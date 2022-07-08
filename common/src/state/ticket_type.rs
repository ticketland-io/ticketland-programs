use anchor_lang::prelude::*;
use super::{
  sale_type::SaleType,
  alias::Slot,
};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Eq, PartialEq, Debug)]
pub struct TicketType {
  pub n_tickets: u32,
  pub sale_type: SaleType,
  pub sale_start_time: Slot,
  pub merkle_root: [u8; 32],
}
