use anchor_lang::prelude::*;
use common::{
  state::ticket_type::TicketType,
};

// Additional space in bytes (1kb) we want to allocate for potential future state expansion
pub const SPACE_MARGIN: usize = 1000;

#[account]
pub struct Sale {
  /// The unique id of the event which this sale is part of
  pub event_id: u64,

  /// A unique index that will differentiate multiple sales of one single event
  pub ticket_type_index: usize,

  /// The ticket type that decides the sale mechanism
  pub ticket_type: TicketType,
}
