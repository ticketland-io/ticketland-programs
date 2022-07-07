use solana_sdk::{
  pubkey::Pubkey,
};
use solana_test_utils::{
  spl::Spl,
};

pub fn event_nft_authority(state: &Pubkey) -> (Pubkey, u8) {
  Pubkey::find_program_address(
    &[b"event_nft_authority", state.as_ref()],
    &event_registry::id()
  )
}

pub fn cpi_authority(state: &Pubkey) -> (Pubkey, u8) {
  Pubkey::find_program_address(
    &[b"cpi_authority", state.as_ref()],
    &event_registry::id()
  )
}
