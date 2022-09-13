use anchor_lang::prelude::*;

#[account]
pub struct Listing {
  pub nft_ticket_metadata: Pubkey,
  pub asking_price: u64,
}
