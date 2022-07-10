use solana_sdk::{
  pubkey::Pubkey,
};
use crate::{
  program_id::ticket_sale_program_id,
};

pub fn ticket_sale_state(
  state: &Pubkey,
  ticket_type_index: usize,
  event_id: u64,
) -> (Pubkey, u8) {
  Pubkey::find_program_address(
    &[
      b"sale",
      state.as_ref(),
      ticket_type_index.to_string().as_ref(),
      &event_id.to_string().as_ref()
    ],
    &ticket_sale_program_id()
  )
}
