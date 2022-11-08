use anchor_lang::prelude::*;

pub const MAX_TICKET_NFT_OPERATORS: usize = 10;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct InitBumps {
  pub nft_authority: u8,
}

#[account]
pub struct State {
  pub bumps: InitBumps,

  /// The Ticket sale Program
  pub ticket_sale_program: Pubkey,

  /// A State account of the Ticket sale Program
  pub ticket_sale_state: Pubkey,

  /// The Secondary Market Program
  pub secondary_market_program: Pubkey,

  /// A State account of the Secondary Market Program
  pub secondary_market_state: Pubkey,

  /// The authority of all NFT Mints that are created in this contract
  pub nft_authority: Pubkey,

  /// The deployer of this instance
  pub deployer: Pubkey,

  /// The array of operators
  pub operators: Vec<Pubkey>,
}
