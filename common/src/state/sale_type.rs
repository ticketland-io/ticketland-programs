use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Eq, PartialEq, Debug)]
pub enum SaleType {
  FixedPrice(u64),
  DutchAuction {
    start_price: u64,
    end_price: u64,
    curve_length: u16,
    drop_interval: u16,
  }
}
