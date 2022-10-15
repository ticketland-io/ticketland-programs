use anchor_lang::prelude::*;

pub const MAX_OPERATORS: usize = 10;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct InitBumps {
  pub cpi_authority: u8,
}

#[account]
#[derive(Default)]
pub struct State {
  pub bumps: InitBumps,
  /// The fees that will be collected by ticketland on every sale
  pub protocol_fee: u16,
  pub treasury: Pubkey,
  pub n_markets: u32,
  pub deployer: Pubkey,
  pub event_registry_state: Pubkey,
  pub event_registry_program: Pubkey,
  pub ticket_sale_state: Pubkey,
  pub ticket_sale_program: Pubkey,
  pub ticket_nft_state: Pubkey,
  pub ticket_nft_program: Pubkey,
  /// The list of all operators that can fill sell listing on users' behalf
  pub operators: Vec<Pubkey>,
}
