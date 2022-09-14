use anchor_lang::prelude::*;

#[account]
pub struct SeatVerification {
  pub bump: u8,
  pub verified: bool,
}
