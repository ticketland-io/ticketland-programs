use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Eq, PartialEq, Debug)]
pub enum SaleType {
  Free,
  FixedPrice {amount: u64},
  Refundable {amount: u64},
  DutchAuction {
    start_price: u64,
    end_price: u64,
    curve_length: u16,
    drop_interval: u16,
  }
}

impl SaleType {
  pub fn is_free(&self) -> bool {
    match self {
      Self::Free => true,
      _ => false
    }
  }

  pub fn is_fixed_price(&self) -> bool {
    match self {
      Self::FixedPrice {..} => true,
      _ => false
    }
  }

  pub fn is_refundable(&self) -> bool {
    match self {
      Self::Refundable {..} => true,
      _ => false
    }
  }

  pub fn is_dutch_auction(&self) -> bool {
    match self {
      Self::DutchAuction {..} => true,
      _ => false
    }
  }
}

#[macro_export]
macro_rules! impl_sale_type {
  () => {
    pub use super::SaleType;
  }
}
