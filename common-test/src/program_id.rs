use solana_sdk::{
  pubkey::Pubkey,
};

pub fn event_registry_program_id() -> Pubkey {
  event_registry::id()
}

pub fn ticket_sale_program_id() -> Pubkey {
  ticket_sale::id()
}

pub fn ticket_nft_program_id() -> Pubkey {
  ticket_nft::id()
}
