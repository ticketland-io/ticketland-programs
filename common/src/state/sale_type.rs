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

impl SaleType {
  pub fn is_fixed_price(&self) -> bool {
    match self {
      Self::FixedPrice(_) => true,
      _ => false
    }
  }

  pub fn is_dutch_auction(&self) -> bool {
    match self {
      Self::DutchAuction{..} => true,
      _ => false
    }
  }
}
