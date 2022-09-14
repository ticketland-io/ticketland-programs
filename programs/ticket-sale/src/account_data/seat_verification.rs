use anchor_lang::prelude::*;

#[account]
pub struct SeatVerification {
  pub verified: bool,
}
