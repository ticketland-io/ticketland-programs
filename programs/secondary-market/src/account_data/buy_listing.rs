use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct BuyListingBumps {
  pub listing_escrow: u8,
}

#[account]
pub struct BuyListing {
  pub bumps: BuyListingBumps,

  /// The price that buyer is willing to pay to purchase a ticket
  pub bid_price: u64,

  /// The buyer that is creating this buy listing
  pub buyer: Pubkey,
}
