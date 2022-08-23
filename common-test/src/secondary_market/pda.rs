use solana_sdk::{
  pubkey::Pubkey,
};
use ticket_nft::account_data::ticket_metadata;
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
  market_id: [u8; 32],
  event_id: [u8; 32],
  ticket_metadata: &Pubkey,
) -> (Pubkey, u8) {
  Pubkey::find_program_address(
    &[b"sell_listing", state.as_ref(), &market_id, &event_id, ticket_metadata.as_ref()],
    &secondary_market_program_id(),
  )
}
