use anchor_lang::prelude::*;
use common::{
  state::ticket_type::TicketType,
};

// Additional space in bytes (1kb) we want to allocate for potential future state expansion
pub const SPACE_MARGIN: usize = 1000;

pub const MAX_TICKET_TYPES: usize = 10;

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
  pub ticket_types: Vec<TicketType>,
  // A bitmap which has n_tickets bits that represent each seat e.g bit at position 0
  pub seats: Vec<u8>,
}
