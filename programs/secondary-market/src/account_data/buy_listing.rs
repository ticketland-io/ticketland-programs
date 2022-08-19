use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct BuyListingBumps {
  pub listing_escrow: u8,
}

#[account]
pub struct BuyListing {
  pub bumps: BuyListingBumps,

  pub market_id: [u8; 32],

  /// The price that buyer is willing to pay to purchase a ticket
  pub bid_price: u64,
}
