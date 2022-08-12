use anchor_lang::prelude::*;
use super::{
  sale_type::SaleType,
};

// l and r are the list and right boundaries of the range which is inclusive.
#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone,  Eq, PartialEq, Default, Debug)]
pub struct SeatRange {
  pub l: u32,
  pub r: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Eq, PartialEq, Debug)]
pub struct TicketType {
  pub n_tickets: u32,
  pub sale_type: SaleType,
  pub sale_start_time: i64,
  pub merkle_root: [u8; 32],
  pub seat_range: SeatRange,
}

#[macro_export]
macro_rules! impl_seat_range {
  () => {
    pub use common::state::ticket_type::SeatRange;
  }
}

#[macro_export]
macro_rules! impl_ticket_type {
  () => {
    pub use common::state::ticket_type::TicketType;
  }
}
