use anchor_lang::prelude::*;

// Additional space in bytes (1kb) we want to allocate for potential future state expansion
pub const SPACE_MARGIN: usize = 1000;

#[account]
pub struct EventCapacity {
  pub event_id: u64,
  // A bitmap which has n_tickets bits that represent each seat e.g bit at position 0
  pub seats: Vec<u8>,
}
