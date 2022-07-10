use anchor_lang::prelude::*;

// Additional space in bytes (1kb) we want to allocate for potential future state expansion
pub const SPACE_MARGIN: usize = 1000;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct InitBumps {
  pub cpi_authority: u8,
}

#[account]
pub struct State {
  pub bumps: InitBumps,

  /// The Event Registry Program
  pub event_registry_program: Pubkey,

  /// A State account of the Event Registry Program
  pub event_registry_state: Pubkey,

  /// This is ticketland.io treasury address
  pub treasury: Pubkey,

  /// The deployer of this instance
  pub deployer: Pubkey,

  /// The account that will be calling other programs
  pub cpi_authority: Pubkey,
}
