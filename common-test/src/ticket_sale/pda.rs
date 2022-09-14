use solana_sdk::{
  pubkey::Pubkey,
};
use crate::{
  program_id::ticket_sale_program_id,
};

pub fn ticket_sale_state(
  state: &Pubkey,
  ticket_type_index: u8,
  event_id: [u8; 32],
) -> (Pubkey, u8) {
  Pubkey::find_program_address(
    &[
      b"sale",
      state.as_ref(),
      ticket_type_index.to_string().as_ref(),
      &event_id
    ],
    &ticket_sale_program_id(),
  )
}

pub fn cpi_authority(state: &Pubkey) -> (Pubkey, u8) {
  Pubkey::find_program_address(
    &[b"ticket_sale:cpi_authority", state.as_ref()],
    &ticket_sale_program_id(),
  )
}

pub fn seat_verification(state: &Pubkey, seat_index: u32, seat_name: &String) -> (Pubkey, u8) {
  Pubkey::find_program_address(
    &[
      b"seat_verification",
      state.as_ref(),
      seat_index.to_string().as_ref(),
      seat_name.as_ref(),  
    ],
    &ticket_sale_program_id(),
  )
}
