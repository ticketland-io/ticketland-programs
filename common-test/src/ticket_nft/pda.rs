use solana_sdk::{
  pubkey::Pubkey,
};
use crate::{
  program_id::ticket_nft_program_id,
};

pub fn nft_authority(state: &Pubkey) -> (Pubkey, u8) {
  Pubkey::find_program_address(
    &[b"nft_authority", state.as_ref()],
    &ticket_nft_program_id()
  )
}

pub fn ticket_nft(
  state: &Pubkey,
  ticket_buyer: &Pubkey,
  event_id: [u8; 32],
) -> (Pubkey, u8) {
  Pubkey::find_program_address(
    &[
      b"ticket_nft",
      state.as_ref(),
      ticket_buyer.as_ref(),
      &event_id
    ],
    &ticket_nft_program_id()
  )
}

pub fn ticket_metadata(state: &Pubkey, ticket_nft: &Pubkey) -> (Pubkey, u8) {
  Pubkey::find_program_address(
    &[b"ticket_metadata", state.as_ref(), ticket_nft.as_ref()],
    &ticket_nft_program_id()
  )
}
