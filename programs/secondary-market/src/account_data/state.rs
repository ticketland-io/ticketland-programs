use anchor_lang::prelude::*;

// Additional space in bytes (1kb) we want to allocate for potential future state expansion
pub const SPACE_MARGIN: usize = 1000;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct InitBumps {
  pub event_nft_authority: u8,
  pub cpi_authority: u8,
}

#[account]
#[derive(Default)]
pub struct State {
  pub protocol_fee: u16,
  pub deployer: Pubkey,
  pub ticket_sale_state: Pubkey,
  pub ticket_sale_program: Pubkey,
}
