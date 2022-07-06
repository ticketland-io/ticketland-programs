use anchor_lang::prelude::*;

// Additional space in bytes (1kb) we want to allocate for potential future state expansion
pub const SPACE_MARGIN: usize = 1000;

pub const MAX_TICKET_TYPES: usize = 10;

pub type SLOT = u64;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct EventBumps {
  pub event: u8,
  pub event_nft: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum SaleType {
  FixedPrice(u64),
  DutchAuction {
    start_price: u64,
    end_price: u64,
    curve_length: u16,
    drop_interval: u16,
  }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TicketType {
  pub n_tickets: u32,
  pub sale_type: SaleType,
  pub sale_start_time: SLOT,
  pub start_time: SLOT,
  pub end_time: SLOT,
  pub merkle_root: [u8; 32],
}

#[account]
pub struct Event {
  pub bumps: EventBumps,
  pub event_organizer: Pubkey,
  pub id: u64,
  pub ticket_types: Vec<TicketType>,
  // A bitmap which has n_tickets bits that represent each seat e.g bit at position 0
  pub seats: Vec<u8>,
}
