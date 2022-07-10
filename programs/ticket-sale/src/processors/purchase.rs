use anchor_lang::prelude::*;
use crate::{
  context::purchase::*,
};

pub fn exec(
  ctx: Context<Purchase>,
  seat_index: u32,
  seat_name: String,
) -> Result<()> {
  // 1. Are there any available seats for this type of ticket
  // 1. Make sure that the given params belong to the Sale's ticket_type sparse MT
  // 2. Check that the seat_index is available
  // 3. Transfer funds
  // 4. CPI to ticket NFT to mint the ticket
  Ok(())
}
