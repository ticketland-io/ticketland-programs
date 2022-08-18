use anchor_lang::prelude::*;
use std::mem::size_of;
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
      buyer_data.n_listing.to_string().as_ref(),
    ],
    bump,
  )]
  pub buy_listing: Account<'info, BuyListing>,

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

  #[account(mut)]
  pub ticket_buyer: Signer<'info>,

  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
}
