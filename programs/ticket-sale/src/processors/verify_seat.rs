use anchor_lang::prelude::*;
use crate::{
  context::verify_seat::*,
  acl::seat_validity,
};

// Make sure that the given params belong to the Sale's ticket_type sparse MT
#[access_control(seat_validity::verify(
  ctx.accounts.sale.ticket_type.merkle_root,
  merkle_proof,
  seat_index,
  &seat_name,
))]
pub fn exec(
  ctx: Context<VerifySeat>,
  seat_index: u32,
  seat_name: String,
  merkle_proof: Vec<[u8; 32]>,
) -> Result<()> {
  let seat_verification = &mut ctx.accounts.seat_verification;
  
  seat_verification.bump = *ctx.bumps.get("seat_verification").unwrap();
  seat_verification.verified = true;

  Ok(())
}
