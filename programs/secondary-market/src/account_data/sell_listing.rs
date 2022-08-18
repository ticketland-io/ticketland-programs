use anchor_lang::prelude::*;

#[account]
pub struct SellListing {
  pub market_id: [u8; 32],

  /// The metadata of the ticket nft that is listed for sale
  pub ticket_metadata: Pubkey,

  /// The listing sell price
  pub ask_price: u64,
}
