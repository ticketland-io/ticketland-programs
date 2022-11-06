use anchor_lang::prelude::*;
use common::state::alias::Slot;

#[account]
pub struct SeatReservation {
  pub valid_until: Slot,
  pub recipient: Pubkey,
}
