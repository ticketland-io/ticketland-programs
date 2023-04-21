use anchor_lang::prelude::*;
use crate::account_data::{
  state::*,
  sell_listing::*,
};

#[derive(Accounts)]
#[instruction(ticket_nft: Pubkey, event_id: [u8; 32])]
pub struct CancelSellListing<'info> {
  // The state account of each instance of this program
  #[account()]
  pub state: Box<Account<'info, State>>,

  /// CHECK: The Event account.
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

  // The sell listing account
  // It will be closed right after the instruction is executed
  #[account(
    mut,
    close = ticket_owner,
    seeds = [
      b"sell_listing",
      state.key().as_ref(),
      &event_id,
      ticket_metadata.key().as_ref(),
    ],
    bump,
  )]
  pub sell_listing: Box<Account<'info, SellListing>>,

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
}
