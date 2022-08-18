use anchor_lang::prelude::*;
use std::mem::size_of;
use anchor_spl::{
  token::{Mint, Token, TokenAccount},
  associated_token::AssociatedToken,
};
use crate::{
  account_data::{
    state::*,
    market::*,
    sell_listing::*,
  },
};

#[derive(Accounts)]
#[instruction(ticket_nft: Pubkey, market_id: [u8; 32], event_id: [u8; 32])]
pub struct CreateSellListing<'info> {
  // The state account of each instance of this program
  #[account()]
  pub state: Account<'info, State>,

  // The market account
  #[account(
    seeds = [
      b"market",
      state.key().as_ref(),
      &event_id,
    ],
    bump = market.bumps.market,
  )]
  pub market: Account<'info, Market>,

  // The sell listing account
  #[account(
    init,
    space = 8 + size_of::<SellListing>(),
    payer = ticket_owner,
    seeds = [
      b"sell_listing",
      state.key().as_ref(),
      &market_id,
      &event_id,
      ticket_metadata.key().as_ref(),
    ],
    bump,
  )]
  pub sell_listing: Account<'info, SellListing>,

  /// CHECK: The ticket metadata account.
  /// Additional checks will take place in the processor
  #[account(
    seeds = [
      b"ticket_metadata",
      state.ticket_nft_state.key().as_ref(),
      ticket_nft.as_ref(),
    ],
    bump,
    seeds::program = state.ticket_nft_program,
  )]
  pub ticket_metadata: AccountInfo<'info>,

  /// CHECK: The token that was used in the primary market of this event
  #[account()]
  pub purchase_token: Account<'info, Mint>,

  /// The event organizer ATA that till be receiving the funds from the ticket sale if purchase token is not SOL
  #[account(
    init_if_needed,
    payer = ticket_owner,
    associated_token::mint = purchase_token,
    associated_token::authority = ticket_owner,
  )]
  pub ticket_owner_purchase_token_ata: Account<'info, TokenAccount>,

  #[account(mut)]
  pub ticket_owner: Signer<'info>,

  pub token_program: Program<'info, Token>,
  pub associated_token_program: Program<'info, AssociatedToken>,
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
}
