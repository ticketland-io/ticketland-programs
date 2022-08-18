use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};
use anchor_safe_math::SafeMath;
use common::{
  utils::bitmap,
  account_data::{
    serialization::deser,
  },
  state::{
    sale_type::*,
  },
};
use crate::{
  context::fixed_price_purchase::*,
  account_data::{
    event::Event,
    event_capacity::MAX_VENUE_CAPACITY,
  },
  acl::seat_validity,
  utils::program_error::ErrorCode,
};

fn transfer_token<'info>(
  ctx: &Context<FixedPricePurchase<'info>>,
  from: AccountInfo<'info>,
  to: AccountInfo<'info>,
  authority: AccountInfo<'info>,
  amount: u64,
) -> Result<()> {
  let cpi_accounts = Transfer::<'info> {from, to, authority};
  let cpi_program = ctx.accounts.token_program.to_account_info();
  let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

  token::transfer(cpi_ctx, amount)
}

/// Transfer the purchase funds to event organizer and our treasury
fn transfer_funds(ctx: &Context<FixedPricePurchase>, event: &Event) -> Result<u64> {
  let ticket_type = &ctx.accounts.sale.ticket_type;
  let amount = if let SaleType::FixedPrice {amount} = ticket_type.sale_type {
    amount
  } else {
    // This should never happen since we already have the same check in the FixedPricePurchase context
    return Err(ErrorCode::UnexpectedSaleAccount.into());
  };
  let (event_organizer_amount, service_fee_amount) = event.currency.calc_fee(amount)?;

  // send to event organizer
  transfer_token(
    &ctx,
    ctx.accounts.ticket_buyer_ata.to_account_info().clone(),
    ctx.accounts.event_organizer_purchase_token_ata.to_account_info().clone(),
    ctx.accounts.ticket_buyer.to_account_info().clone(),
    event_organizer_amount,
  )?;

  // send to treasury
  transfer_token(
    &ctx,
    ctx.accounts.ticket_buyer_ata.to_account_info().clone(),
    ctx.accounts.service_fee_ata.to_account_info().clone(),
    ctx.accounts.ticket_buyer.to_account_info().clone(),
    service_fee_amount,
  )?;

  Ok(amount)
}

fn mint_ticket(ctx: &Context<FixedPricePurchase>, price_sold: u64, seat_index: u32, seat_name: String) -> Result<()> {
  let cpi_program = ctx.accounts.ticket_nft_program.to_account_info();
  let cpi_accounts = ticket_nft::cpi::accounts::CreateTicket {
    state: ctx.accounts.ticket_nft_program_state.to_account_info(),
    ticket_metadata: ctx.accounts.ticket_metadata.to_account_info(),
    nft_authority: ctx.accounts.nft_authority.to_account_info(),
    nft: ctx.accounts.ticket_nft.to_account_info(),
    event_nft_metadata: ctx.accounts.event_nft_metadata.to_account_info(),
    metadata: ctx.accounts.ticket_metaplex_metadata.to_account_info(),
    master_edition: ctx.accounts.master_edition.to_account_info(),
    ticket_nft_ata: ctx.accounts.ticket_nft_ata.to_account_info(),
    ticket_sale_cpi_authority: ctx.accounts.cpi_authority.to_account_info(),
    ticket_buyer: ctx.accounts.ticket_buyer.to_account_info(),
    token_program: ctx.accounts.token_program.to_account_info(),
    associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
    metadata_program: ctx.accounts.metadata_program.to_account_info(),
    system_program: ctx.accounts.system_program.to_account_info(),
    rent: ctx.accounts.rent.to_account_info(),
  };
  
  let state = &ctx.accounts.state;
  let state_key = state.key();
  let seeds: &[&[u8]] = &[
    b"ticket_sale:cpi_authority", state_key.as_ref(),
    &[state.bumps.cpi_authority]
  ];
  let signer_seeds:&[&[&[u8]]] = &[&seeds[..]];

  let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

  ticket_nft::cpi::create_ticket(
    cpi_ctx,
		ctx.accounts.state.bumps.cpi_authority,
		ctx.accounts.sale.event_id,
    seat_index,
    ctx.accounts.sale.key(),
    price_sold,
		seat_name,
  )?;

  Ok(())
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
fn account_checks(ctx: &Context<FixedPricePurchase>, event: &Event) -> Result<()>  {
  let sale = &ctx.accounts.sale;

  require!(event.id == sale.event_id, ErrorCode::WrongEventAccount);
  require!(event.currency.mint_account == ctx.accounts.purchase_token.key(), ErrorCode::UnsupportedPurchaseToken);
  require!(event.event_organizer == ctx.accounts.event_organizer.key(), ErrorCode::WrongEventOrganizer);
  require!(event.event_capacity == ctx.accounts.event_capacity.key(), ErrorCode::WrongEventCapacityAccount);
  
  Ok(())
}

// 1. Make sure that the given params belong to the Sale's ticket_type sparse MT
#[access_control(seat_validity::verify(
  ctx.accounts.sale.ticket_type.merkle_root,
  merkle_proof,
  seat_index,
  &seat_name,
))]
pub fn exec(
  ctx: Context<FixedPricePurchase>,
  seat_index: u32,
  seat_name: String,
  merkle_proof: Vec<[u8; 32]>,
) -> Result<()> {
  let event: Event = deser(ctx.accounts.event.clone())?;

  account_checks(&ctx, &event)?;

  // 2. Has sale started?
  let sale = &ctx.accounts.sale;
  // TODO: Use an oracle to get the current time
  require!(Clock::get().unwrap().unix_timestamp >= sale.ticket_type.sale_start_time, ErrorCode::SaleNotStarted);
  require!(Clock::get().unwrap().unix_timestamp <= sale.ticket_type.sale_end_time, ErrorCode::SaleFinished);

  // 3. Are there any available seats for this type of ticket
  let event_capacity = &mut ctx.accounts.event_capacity.load_mut()?;
  require!(event_capacity.available_tickets > 0, ErrorCode::TicketSoldOut);

  // 4. Check that the seat_index is available
  require!(
    !bitmap::is_set::<MAX_VENUE_CAPACITY>(seat_index, &event_capacity.seats),
    ErrorCode::SeatNotAvailable,
  );

  // 5. Transfer funds
  let price_sold = transfer_funds(&ctx, &event)?;

  // 6. CPI to Ticket NFT program to mint the ticket
  mint_ticket(&ctx, price_sold, seat_index, seat_name)?;

  // 7. Update state
  bitmap::flip_bit::<MAX_VENUE_CAPACITY>(seat_index, &mut event_capacity.seats);

  // - total tickets sold (Ticket Sale State account data)
  let state = &mut ctx.accounts.state;
  state.total_sold = state.total_sold.safe_add(1)?;

  // - decrease available_tickets
  let available_tickets = event_capacity.available_tickets; // use local to avoid reference to packed field is unaligned
  event_capacity.available_tickets = available_tickets.safe_sub(1)?;

  Ok(())
}
