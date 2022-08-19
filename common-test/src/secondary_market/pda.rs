use solana_sdk::{
  pubkey::Pubkey,
};
use crate::{
  program_id::secondary_market_program_id,
};

pub fn cpi_authority(state: &Pubkey) -> (Pubkey, u8) {
  Pubkey::find_program_address(
    &[b"market:cpi_authority", state.as_ref()],
    &secondary_market_program_id(),
  )
}
