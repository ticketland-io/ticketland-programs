use anchor_lang::prelude::*;
use crate::{
  context::purchase::*,
  acl::seat_validity,
  utils::program_error::ErrorCode,
};

// 1. Make sure that the given params belong to the Sale's ticket_type sparse MT
#[access_control(seat_validity::verify(
  ctx.accounts.sale.ticket_type.merkle_root,
  merkle_proof,
  seat_index,
  seat_name,
))]
pub fn exec(
  ctx: Context<Purchase>,
  seat_index: u32,
  seat_name: String,
  merkle_proof: Vec<[u8; 32]>,
) -> Result<()> {
  let event_capacity = &ctx.accounts.event_capacity.load()?;
  // 2. Are there any available seats for this type of ticket
  require!(event_capacity.available_tickets > 0, ErrorCode::TicketSoldOut);

  // 3. Check that the seat_index is available


  // 3. Transfer funds
  // 4. CPI to ticket NFT to mint the ticket
  Ok(())
}
