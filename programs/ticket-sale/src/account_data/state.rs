use anchor_lang::prelude::*;

// Additional space in bytes (1kb) we want to allocate for potential future state expansion
pub const SPACE_MARGIN: usize = 1000;


#[account]
pub struct State {
  /// The Event Registry Program
  pub event_registry_program: Pubkey,

  /// A State account of the Event Registry Program
  pub event_registry_state: Pubkey,

  /// The deployer of this instance
  pub deployer: Pubkey,
}
