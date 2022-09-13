use anchor_lang::prelude::*;
use crate::{
  state::{
    ticket_type::TicketType,
    currency::*,
  },
};

pub const MAX_TICKET_TYPES: usize = 10;
pub const MAX_SEAT_RANGES: usize = 10;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct EventBumps {
  pub event: u8,
  pub event_nft: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Event {
  pub id: [u8; 32],
  pub bumps: EventBumps,
  pub event_organizer: Pubkey,
  pub event_capacity: Pubkey,
  pub n_tickets: u32,
  pub start_time: i64,
  pub end_time: i64,
  // The token that will be used to purchase the tickets. At the moment we support multiple currencies
  // but event organizers need to choose one per event. This will change in the future.
  pub currency: Currency,
  pub event_organizer_purchase_token_ata: Pubkey,
  pub ticket_types: Vec<TicketType>,
}
