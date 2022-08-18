use anchor_lang::prelude::*;

#[account]
pub struct BuyListing {
  pub market_id: [u8; 32],

  /// The price that buyer is willing to pay to purchase a ticket
  pub bid_price: u64,
}
