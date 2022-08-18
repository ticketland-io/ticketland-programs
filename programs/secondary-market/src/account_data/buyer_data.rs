use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct BuyerData {
  /// Useful to count how many buy listing has a 
  pub n_listing: u16,
}
