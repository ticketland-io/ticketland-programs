use anchor_lang::prelude::*;
use common::{
  utils::bitmap,
  token::is_wrapped_sol,
};
use crate::{
  context::fixed_price_purchase::*,
  account_data::event_capacity::MAX_VENUE_CAPACITY,
  acl::seat_validity,
  utils::program_error::ErrorCode,
};

/// Transfer the purchase funds to event organizer and our treasury
fn transfer_funds(ctx: &Context<FixedPricePurchase>) -> Result<()> {
  // let (event_organizer_amount, service_fee_amount) = 


  // if is_wrapped_sol(ctx.accounts.purchase_token.key()) {
  //   transfer_sol(&ctx)?;
  // } else {
  //   transfer(&ctx)?;
  // }

  Ok(())
}

// 1. Make sure that the given params belong to the Sale's ticket_type sparse MT
#[access_control(seat_validity::verify(
  ctx.accounts.sale.ticket_type.merkle_root,
  merkle_proof,
  seat_index,
  seat_name,
))]
pub fn exec(
  ctx: Context<FixedPricePurchase>,
  seat_index: u32,
  seat_name: String,
  merkle_proof: Vec<[u8; 32]>,
) -> Result<()> {
  // 2. Has sale started?
  let sale = &ctx.accounts.sale;
  require!(Clock::get().unwrap().slot >= sale.ticket_type.sale_start_time, ErrorCode::SaleNotStarted);

  let event_capacity = &mut ctx.accounts.event_capacity.load_mut()?;
  // 3. Are there any available seats for this type of ticket
  require!(event_capacity.available_tickets > 0, ErrorCode::TicketSoldOut);

  // 4. Check that the seat_index is available
  require!(
    bitmap::is_true::<u8, MAX_VENUE_CAPACITY>(seat_index, &event_capacity.seats),
    ErrorCode::SeatNotAvailable,
  );

  // 5. Transfer funds
  transfer_funds(&ctx)?;

  // 6. CPI to ticket NFT to mint the ticket
  Ok(())
}
