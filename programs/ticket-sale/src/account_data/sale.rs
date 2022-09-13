use anchor_lang::prelude::*;
use common::{
  state::{
    ticket_type::TicketType,
  },
};

#[account]
pub struct Sale {
  pub bump: u8,

  /// The unique id of the event which this sale is part of
  pub event_id: [u8; 32],

  /// A unique index that will differentiate multiple sales of one single event
  pub ticket_type_index: u8,

  /// The ticket type that decides the sale mechanism
  pub ticket_type: TicketType,
}
