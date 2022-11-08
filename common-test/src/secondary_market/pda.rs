use solana_sdk::{
  pubkey::Pubkey,
};
use crate::{
  program_id::secondary_market_program_id,
};

pub fn market(state: &Pubkey, event_id: [u8; 32]) -> (Pubkey, u8) {
  Pubkey::find_program_address(
    &[b"market", state.as_ref(), &event_id],
    &secondary_market_program_id(),
  )
}

pub fn cpi_authority(state: &Pubkey) -> (Pubkey, u8) {
  Pubkey::find_program_address(
    &[b"market:cpi_authority", state.as_ref()],
    &secondary_market_program_id(),
  )
}

pub fn sell_listing(
  state: &Pubkey,
  event_id: [u8; 32],
  ticket_metadata: &Pubkey,
) -> (Pubkey, u8) {
  Pubkey::find_program_address(
    &[b"sell_listing", state.as_ref(), &event_id, ticket_metadata.as_ref()],
    &secondary_market_program_id(),
  )
}

pub fn buy_listing(
  state: &Pubkey,
  event_id: [u8; 32],
  ticket_buyer: &Pubkey,
  n_listing: u16,
) -> (Pubkey, u8) {
  Pubkey::find_program_address(
    &[b"buy_listing", state.as_ref(), &event_id, ticket_buyer.as_ref(), n_listing.to_string().as_ref()],
    &secondary_market_program_id(),
  )
}

pub fn buyer_data(
  state: &Pubkey,
  event_id: [u8; 32],
  ticket_buyer: &Pubkey,
) -> (Pubkey, u8) {
  Pubkey::find_program_address(
    &[b"buyer_data", state.as_ref(), &event_id, ticket_buyer.as_ref()],
    &secondary_market_program_id(),
  )
}

pub fn listing_escrow(
  state: &Pubkey,
  event_id: [u8; 32],
  buy_listing: &Pubkey,
) -> (Pubkey, u8) {
  Pubkey::find_program_address(
    &[b"listing_escrow", state.as_ref(), &event_id, buy_listing.as_ref()],
    &secondary_market_program_id(),
  )
}

pub fn sell_listing_reservation(sell_listing: &Pubkey,) -> (Pubkey, u8) {
  Pubkey::find_program_address(
    &[b"sell_listing_reservation", sell_listing.as_ref()],
    &secondary_market_program_id(),
  )
}
