use anchor_lang::prelude::*;

pub const MAX_CURRENCY_SUPPORT: usize = 10;

// Additional space in bytes (1kb) we want to allocate for potential future state expansion
pub const SPACE_MARGIN: usize = 1000;

pub type SLOT = u64;

#[account]
pub struct Event {
  pub creator: Pubkey,
  pub start_time: SLOT,
  pub end_time: SLOT,
}
