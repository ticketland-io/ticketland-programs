use anchor_lang::prelude::*;

pub const MAX_CURRENCY_SUPPORT: usize = 10;

// Additional space in bytes (1kb) we want to allocate for potential future state expansion
pub const SPACE_MARGIN: usize = 1000;

pub type SLOT = u64;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct EventBumps {
  pub event: u8,
  pub event_nft: u8,
}

#[account]
pub struct Event {
  pub bumps: EventBumps,
  pub event_organizer: Pubkey,
  pub id: u64,
  pub start_time: SLOT,
  pub end_time: SLOT,
}
