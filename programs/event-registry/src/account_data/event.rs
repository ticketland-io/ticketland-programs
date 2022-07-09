use anchor_lang::prelude::*;
use common::{
  state::{
    ticket_type::TicketType,
    alias::*,
  },
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
  pub event_capacity: Pubkey,
  pub n_tickets: u32,
  pub id: u64,
  pub start_time: Slot,
  pub end_time: Slot,
  pub sale_start_time: Option<Vec<Slot>>,
  pub ticket_types: Vec<TicketType>,
}
