use anchor_lang::prelude::*;
use anchor_spl::{
  token::{Mint, Token, TokenAccount},
  associated_token::AssociatedToken,
};
use crate::{
  account_data::{
    state::*,
    buy_listing::*,
  },
};

#[derive(Accounts)]
#[instruction(n_listing: u16, event_id: [u8; 32])]
pub struct CancelBuyListing<'info> {
  #[account(mut)]
  pub state: Box<Account<'info, State>>,

  // The buy listing account
  // It will be closed right after the instruction is executed
  #[account(
    mut,
    close = ticket_buyer,
    seeds = [
      b"buy_listing",
      state.key().as_ref(),
      &event_id,
      ticket_buyer.key().as_ref(),
      n_listing.to_string().as_ref(),
    ],
    bump,
  )]
  pub buy_listing: Account<'info, BuyListing>,

  /// CHECK: The Event account
  #[account(
    seeds = [
      b"event",
      state.event_registry_state.key().as_ref(),
      &event_id
    ],
    bump,
    seeds::program = state.event_registry_program,
  )]
  pub event: AccountInfo<'info>,

  /// CHECK: The token that was used in the primary market of this event
  /// Additional checks take place in the processor
  #[account()]
  pub purchase_token: Box<Account<'info, Mint>>,

  /// CHECK: The account that is the authority of the vault ATA that is holding the escrowed funds for the purchase
  #[account(
    mut,
    seeds = [
      b"listing_escrow",
      state.key().as_ref(),
      &event_id,
      buy_listing.key().as_ref(),
    ],
    bump = buy_listing.bumps.listing_escrow,
  )]
  pub listing_escrow: AccountInfo<'info>,

  #[account(
    mut,
    associated_token::mint = purchase_token,
    associated_token::authority = listing_escrow,
  )]
  pub listing_escrow_ata: Box<Account<'info, TokenAccount>>,

  #[account(mut)]
  pub ticket_buyer: Signer<'info>,

  #[account(
    mut,
    associated_token::mint = purchase_token,
    associated_token::authority = ticket_buyer,
  )]
  pub ticket_buyer_ata: Box<Account<'info, TokenAccount>>,

  pub token_program: Program<'info, Token>,
  pub associated_token_program: Program<'info, AssociatedToken>,
}
