use anchor_lang::prelude::*;

// Additional space in bytes (1kb) we want to allocate for potential future state expansion
pub const SPACE_MARGIN: usize = 1000;

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

  /// The authority of all NFT Mints that are created in this contract
  pub nft_authority: Pubkey,

  /// The deployer of this instance
  pub deployer: Pubkey,
}
