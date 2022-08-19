use anchor_lang::prelude::*;
use std::mem::size_of;
use anchor_spl::{
  token::{Mint, Token, TokenAccount},
  associated_token::AssociatedToken,
};
use crate::{
  account_data::{
    state::*,
    buy_listing::*,
    buyer_data::*,
  },
};

#[derive(Accounts)]
#[instruction(market_id: [u8; 32], event_id: [u8; 32])]
pub struct CreateBuyListing<'info> {
  // The state account of each instance of this program
  #[account()]
  pub state: Account<'info, State>,

  // The buy_listing account
  #[account(
    init_if_needed,
    space = 8 + size_of::<BuyerData>(),
    payer = ticket_buyer,
    seeds = [
      b"buyer_data",
      state.key().as_ref(),
      &market_id,
      &event_id,
      ticket_buyer.key().as_ref(),
    ],
    bump,
  )]
  pub buyer_data: Account<'info, BuyerData>,

  // The buy_listing account
  #[account(
    init,
    space = 8 + size_of::<BuyListing>(),
    payer = ticket_buyer,
    seeds = [
      b"buy_listing",
      state.key().as_ref(),
      &market_id,
      &event_id,
      ticket_buyer.key().as_ref(),
      buyer_data.n_listing.to_string().as_ref(),
    ],
    bump,
  )]
  pub buy_listing: Account<'info, BuyListing>,

  /// CHECK: The account that will be the authority of the vault ATA that will be holding the escrowed funds for the purchase
  #[account(
    init,
    space = 0,
    payer = ticket_buyer,
    seeds = [
      b"listing_vault",
      state.key().as_ref(),
      &market_id,
      &event_id,
      buy_listing.key().as_ref(),
    ],
    bump,
  )]
  pub listing_escrow: AccountInfo<'info>,

  #[account(
    init,
    payer = ticket_buyer,
    associated_token::mint = purchase_token,
    associated_token::authority = listing_escrow,
  )]
  pub listing_escrow_ata: Box<Account<'info, TokenAccount>>,

  /// CHECK: The token that was used in the primary market of this event
  #[account()]
  pub purchase_token: Box<Account<'info, Mint>>,

  #[account(
    mut,
    associated_token::mint = purchase_token,
    associated_token::authority = ticket_buyer,
  )]
  pub ticket_buyer_ata: Box<Account<'info, TokenAccount>>,

  #[account(mut)]
  pub ticket_buyer: Signer<'info>,

  pub token_program: Program<'info, Token>,
  pub associated_token_program: Program<'info, AssociatedToken>,
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
}
