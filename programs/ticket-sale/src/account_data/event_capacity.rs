use anchor_lang::prelude::*;

// Additional space in bytes (1kb) we want to allocate for potential future state expansion
pub const SPACE_MARGIN: usize = 1000;

// With 12500 bytes we can represent up to 12500 * 8 = 100_000 seats
pub const MAX_VENUE_CAPACITY: usize = 12_500;

#[account(zero_copy)]
pub struct EventCapacity {
  pub event_id: [u8; 32],
  pub is_initialized: bool,
  pub available_tickets: u32,

  // A bitmap which has n_tickets bits that represent each seat
  // By default all bits are 0. When a ticket at ticket index N (Nth bit) is purchased
  // then the bit is flipped to 1 indicating that the seat is not available
  pub seats: [u8; MAX_VENUE_CAPACITY],
}
