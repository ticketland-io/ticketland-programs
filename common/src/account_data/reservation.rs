use anchor_lang::prelude::*;
use crate::state::alias::Slot;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Reservation {
  pub valid_until: Slot,
  pub recipient: Pubkey,
}
