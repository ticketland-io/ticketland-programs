use anchor_lang::prelude::*;

#[account]
pub struct EventCapacity {
  pub event_id: [u8; 32],
  pub is_initialized: bool,
  pub available_tickets: u32,

  // A bitmap which has n_tickets bits that represent each seat
  // By default all bits are 0. When a ticket at ticket index N (Nth bit) is purchased
  // then the bit is flipped to 1 indicating that the seat is not available
  // Bitmap allows us to store compact data e.g With 12500 bytes we can represent up to 12500 * 8 = 100_000 seats
  pub seats: Vec<u8>,
}
