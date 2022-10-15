use anchor_lang::prelude::*;
use anchor_safe_math::SafeMath;
use common::{
  utils::bitmap,
};
use crate::{
  context::fixed_price_purchase::*,
  context::free_purchase::*,
  account_data::{
    state::State,
    event::Event,
    event_capacity::EventCapacity,
    sale::Sale,
  },
  utils::program_error::ErrorCode,
};

/// The main issue stems from the fact that we can't have the following account in the FixedPricePurchase context
///  
/// `pub event: Box<Account<'info, Event>>`
/// 
/// The reason is how Anchor does the account checks. For details check https://docs.rs/anchor-lang/0.25.0/anchor_lang/accounts/account/struct.Account.html
/// In essence, anchor will try to check that `Account.info.owner == Event::owner()` is true.
/// Event::owner is by default crate::ID that is the id of the ticket sale program. However, the account itself was created
/// in the Event Registry Program and thus `Account.info.owner` will have that program's address.
/// We could implement the `Owner` trait https://docs.rs/anchor-lang/0.25.0/anchor_lang/trait.Owner.html and return the 
/// Event Registry program address, but that would mean we would have to hard code that address which is against the flexibility
/// we've provided by storing the Event Registry program address in `state` account of this program.
/// For this reason we will do a few manual checks that we did the declarative constraint macro in the Context.
/// 
/// Also not that we don't have to check if ctx.accounts.event.owner == &state.event_registry_program
/// because we already have this constraint in the PDA seeds::program = state.event_registry_program
fn account_checks(
  event_organizer: Pubkey,
  event_capacity: Pubkey,
  event: &Event,
  event_id: [u8; 32],
) -> Result<()>  {
  require!(event.id == event_id, ErrorCode::WrongEventAccount);
  require!(event.event_organizer == event_organizer.key(), ErrorCode::WrongEventOrganizer);
  require!(event.event_capacity == event_capacity.key(), ErrorCode::WrongEventCapacityAccount);
  
  Ok(())
}

pub fn pre_checks<'info>(
  sale: &Box<Account<'info, Sale>>,
  event_capacity: &Account<'info, EventCapacity>,
  seat_index: u32,
) -> Result<()> {
  // 1. Has sale started?

  // TODO: Use an oracle to get the current time
  require!(Clock::get().unwrap().unix_timestamp >= sale.ticket_type.sale_start_time, ErrorCode::SaleNotStarted);
  require!(Clock::get().unwrap().unix_timestamp <= sale.ticket_type.sale_end_time, ErrorCode::SaleFinished);

  // 2. Are there any available seats for this type of ticket
  require!(event_capacity.available_tickets > 0, ErrorCode::TicketSoldOut);

  // 3. Check that the seat_index is available
  require!(
    !bitmap::is_set(seat_index, &event_capacity.seats),
    ErrorCode::SeatNotAvailable,
  );

  Ok(())
}

pub fn fixed_price_purchase_pre_checks(ctx: &Context<FixedPricePurchase>, event: &Event, seat_index: u32) -> Result<()> {
  require!(event.currency.mint_account == ctx.accounts.purchase_token.key(), ErrorCode::UnsupportedPurchaseToken);
  account_checks(
    ctx.accounts.event_organizer.key(),
    ctx.accounts.event_capacity.key(),
    &event,
    ctx.accounts.sale.event_id,
  )?;
  pre_checks(&ctx.accounts.sale, &ctx.accounts.event_capacity, seat_index)?;

  Ok(())
}

pub fn free_purchase_pre_checks(ctx: &Context<FreePurchase>, event: &Event, seat_index: u32) -> Result<()> {
  account_checks(
    ctx.accounts.event_organizer.key(),
    ctx.accounts.event_capacity.key(),
    &event,
    ctx.accounts.sale.event_id,
  )?;
  pre_checks(&ctx.accounts.sale, &ctx.accounts.event_capacity, seat_index)?;

  Ok(())
}

pub fn post_checks<'info>(
  state: &mut Box<Account<'info, State>>,
  event_capacity: &mut Account<'info, EventCapacity>,
  seat_index: u32,
) -> Result<()> {
  // 5. Update state
  bitmap::flip_bit(seat_index, &mut event_capacity.seats);
  
  // - total tickets sold (Ticket Sale State account data)
  state.total_sold = state.total_sold.safe_add(1)?;

  // - decrease available_tickets
  let available_tickets = event_capacity.available_tickets; // use local to avoid reference to packed field is unaligned
  event_capacity.available_tickets = available_tickets.safe_sub(1)?;

  Ok(())
}
