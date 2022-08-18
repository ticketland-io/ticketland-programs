use anchor_lang::prelude::*;
use anchor_spl::{
  token::{Mint, Token, TokenAccount},
  associated_token::AssociatedToken,
};
use crate::{
  account_data::{
    state::*,
    sell_listing::*,
    market::Market,
  },
  utils::{
    program_error::ErrorCode,
  }
};

#[derive(Accounts)]
#[instruction(ticket_nft: Pubkey, market_id: [u8; 32], event_id: [u8; 32])]
pub struct FillSellListing<'info> {
  #[account(mut)]
  pub state: Box<Account<'info, State>>,

  // The sell listing account
  #[account(
    seeds = [
      b"sell_listing",
      state.key().as_ref(),
      &market_id,
      &event_id,
      ticket_nft.as_ref(),
    ],
    bump,
  )]
  pub sell_listing: Box<Account<'info, SellListing>>,

  // The market account
  #[account(
    seeds = [
      b"market",
      state.key().as_ref(),
      &event_id,
    ],
    bump = market.bumps.market,
  )]
  pub market: Box<Account<'info, Market>>,

  /// CHECK: The Event account.
  /// Constraints will be checked in the processor fn
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
  #[account()]
  pub purchase_token: Box<Account<'info, Mint>>,
    
  /// The event organizer ATA that till be receiving the funds from the ticket sale if purchase token is not SOL
  #[account(
    mut,
    constraint = sell_listing.ticket_owner_purchase_token_ata == ticket_owner_purchase_token_ata.key() @ ErrorCode::WrongTicketOwnerPurchaseTokenAta
  )]
  pub ticket_owner_purchase_token_ata: Box<Account<'info, TokenAccount>>,

  #[account(
    mut,
    seeds = [b"cpi_authority", state.event_registry_state.as_ref()],
    bump,
    seeds::program = state.event_registry_program,
  )]
  pub event_registry_cpi_authority: Signer<'info>,

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
