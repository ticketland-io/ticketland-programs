use anchor_lang::prelude::*;
use std::mem::size_of;
use crate::{
  account_data::{
    state::*,
    sell_listing::*,
  },
};

#[derive(Accounts)]
#[instruction(market_id: [u8; 32], event_id: [u8; 32], ticket_nft: Pubkey)]
pub struct CreateSellListing<'info> {
  // The state account of each instance of this program
  #[account()]
  pub state: Account<'info, State>,

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

  /// CHECK: The Event account.
  /// Additional checks will take place in the processor
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

  #[account(mut)]
  pub ticket_owner: Signer<'info>,

  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
}
