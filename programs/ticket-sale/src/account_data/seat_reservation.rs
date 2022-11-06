use anchor_lang::prelude::*;
use common::state::alias::Slot;

#[account]
pub struct SeatReservation {
  pub created: Slot,
  pub recipient: Pubkey,
}
