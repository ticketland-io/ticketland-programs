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
