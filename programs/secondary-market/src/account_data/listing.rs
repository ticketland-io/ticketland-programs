use anchor_lang::prelude::*;

// Additional space in bytes (1kb) we want to allocate for potential future state expansion
pub const SPACE_MARGIN: usize = 1000;

#[account]
pub struct Listing {
  pub nft_ticket_metadata: Pubkey,
  pub asking_price: u64,
}
