use anchor_lang::prelude::*;
use anchor_lang::AccountsClose;
use anchor_safe_math::SafeMath;
use anchor_spl::{
  token::{Token},
  associated_token::AssociatedToken,
};
use ticket_nft::{
  program::TicketNft,
};
use common::{
  utils::bitmap,
};
use crate::{
  account_data::{
    state::State,
    event::Event,
    event_capacity::EventCapacity,
    sale::Sale,
    seat_reservation::SeatReservation,
  },
  utils::program_error::ErrorCode,
};

#[macro_export]
macro_rules! expand_pre_checks {
  ($ctx:ident, $event:ident, $seat_index:ident, $operator:expr) => {
    super::common_purchase::seat_reservation_checks(
      &$ctx.accounts.seat_reservation,
      $operator.to_account_info(),
      $ctx.accounts.ticket_buyer.key(),
    )?;

    super::common_purchase::account_checks(
      $ctx.accounts.event_organizer.key(),
      $ctx.accounts.event_capacity.key(),
      &$event,
      $ctx.accounts.sale.event_id,
    )?;

    super::common_purchase::pre_checks(&$ctx.accounts.sale, &$ctx.accounts.event_capacity, $seat_index)?;
  }
}

#[macro_export]
macro_rules! expand_mint_ticket {
  ($ctx:ident, $recipient:ident, $price_sold:expr, $seat_index:ident,  $seat_name:ident) => {
    super::common_purchase::mint_ticket(
      $recipient,
      $price_sold,
      $seat_index,
      $seat_name,
      &$ctx.accounts.state,
      &$ctx.accounts.sale,
      &$ctx.accounts.ticket_nft_program,
      &$ctx.accounts.ticket_nft_program_state,
      &$ctx.accounts.ticket_metadata,
      &$ctx.accounts.nft_authority,
      &$ctx.accounts.ticket_nft,
      &$ctx.accounts.event_nft_metadata,
      &$ctx.accounts.ticket_nft_ata,
      &$ctx.accounts.cpi_authority,
      &$ctx.accounts.ticket_buyer,
      &$ctx.accounts.token_program,
      &$ctx.accounts.associated_token_program,
      &$ctx.accounts.system_program,
      &$ctx.accounts.rent,
    )?;
  
  }
}

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
pub fn account_checks(
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

pub fn seat_reservation_checks<'info>(
  seat_reservation: &AccountInfo<'info>,
  operator: AccountInfo<'info>,
  ticket_buyer: Pubkey,
) -> Result<()> {
  // if account exists then check if it has expired or the sender is the recipient
  if seat_reservation.lamports() != 0 {
    // This will no fail because it lamports is no 0. It will also check that seat_reservation account
    // is owned by the TicketSale program
    let seat_reservation = Account::<SeatReservation>::try_from(&seat_reservation)?;

    if seat_reservation.recipient == ticket_buyer || Clock::get().unwrap().slot > seat_reservation.valid_until {
      seat_reservation.close(operator)?;
    } else {
      return Err(ErrorCode::SeatReserved.into())
    }
  }

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

pub fn mint_ticket<'info>(
  recipient: Pubkey,
  price_sold: u64,
  seat_index: u32,
  seat_name: String,
  state: &Box<Account<'info, State>>,
  sale: &Box<Account<'info, Sale>>,
  ticket_nft_program: &Program<'info, TicketNft>,
  ticket_nft_program_state: &AccountInfo<'info>,
  ticket_metadata: &AccountInfo<'info>,
  nft_authority: &AccountInfo<'info>,
  ticket_nft: &AccountInfo<'info>,
  event_nft_metadata: &AccountInfo<'info>,
  ticket_nft_ata: &AccountInfo<'info>,
  cpi_authority: &AccountInfo<'info>,
  ticket_buyer: &Signer<'info>,
  token_program: &Program<'info, Token>,
  associated_token_program: &Program<'info, AssociatedToken>,
  system_program: &Program<'info, System>,
  rent: &Sysvar<'info, Rent>,
) -> Result<()> {
  let cpi_program = ticket_nft_program.to_account_info();
  let cpi_accounts = ticket_nft::cpi::accounts::CreateTicket {
    state: ticket_nft_program_state.to_account_info(),
    ticket_metadata: ticket_metadata.to_account_info(),
    nft_authority: nft_authority.to_account_info(),
    nft: ticket_nft.to_account_info(),
    event_nft_metadata: event_nft_metadata.to_account_info(),
    ticket_nft_ata: ticket_nft_ata.to_account_info(),
    ticket_sale_cpi_authority: cpi_authority.to_account_info(),
    ticket_buyer: ticket_buyer.to_account_info(),
    token_program: token_program.to_account_info(),
    associated_token_program: associated_token_program.to_account_info(),
    system_program: system_program.to_account_info(),
    rent: rent.to_account_info(),
  };
  
  let state_key = state.key();
  let seeds: &[&[u8]] = &[
    b"ticket_sale:cpi_authority", state_key.as_ref(),
    &[state.bumps.cpi_authority]
  ];
  let signer_seeds:&[&[&[u8]]] = &[&seeds[..]];

  let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

  ticket_nft::cpi::create_ticket(
    cpi_ctx,
		state.bumps.cpi_authority,
    sale.ticket_type_index,
		sale.event_id,
    seat_index,
    recipient,
    sale.key(),
    price_sold,
		seat_name,
  )?;

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
