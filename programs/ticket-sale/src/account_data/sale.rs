use anchor_lang::prelude::*;
use common::{
  state::ticket_type::TicketType,
};

// Additional space in bytes (1kb) we want to allocate for potential future state expansion
pub const SPACE_MARGIN: usize = 1000;

#[account]
pub struct Sale {
  /// The ticket type that decides the sale mechanism
  pub ticket_type: TicketType,
}
