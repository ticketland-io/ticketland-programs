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

pub fn event(state: &Pubkey, event_id: [u8; 32]) -> (Pubkey, u8) {
  Pubkey::find_program_address(
    &[b"event", state.as_ref(), &event_id],
    &event_registry::id()
  )
}

pub fn event_nft(state: &Pubkey, event_id: [u8; 32]) -> (Pubkey, u8) {
  Pubkey::find_program_address(
    &[b"event_nft", state.as_ref(), &event_id],
    &event_registry::id()
  )
}

pub fn fund_manager(state: &Pubkey, event: &Pubkey, event_organizer: &Pubkey) -> (Pubkey, u8) {
  Pubkey::find_program_address(
    &[b"fund_manager", state.as_ref(), &event.as_ref(), &event_organizer.as_ref()],
    &event_registry::id()
  )
}

pub fn event_organizer_ata(authority: &Pubkey, mint: &Pubkey) -> Pubkey {
  Spl::get_associated_token_address(authority, mint)
}

pub fn organizer_event_nft_ata(authority: &Pubkey, mint: &Pubkey) -> Pubkey {
  Spl::get_associated_token_address(authority, mint)
}

pub fn fund_manager_ata(authority: &Pubkey, mint: &Pubkey) -> Pubkey {
  Spl::get_associated_token_address(authority, mint)
}


