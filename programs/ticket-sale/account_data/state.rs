use anchor_lang::prelude::*;
use common::{
  state::ticket_type::TicketType,
};

// Additional space in bytes (1kb) we want to allocate for potential future state expansion
pub const SPACE_MARGIN: usize = 1000;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct InitBumps {
}

#[account]
pub struct State {
  pub bumps: InitBumps,

  /// The Event Registry Program
  pub event_registry_program: Pubkey,
}
