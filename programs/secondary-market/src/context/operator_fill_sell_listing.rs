use anchor_lang::prelude::*;
use ticket_nft::{
  program::TicketNft,
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
#[instruction(event_id: [u8; 32])]
pub struct OperatorFillSellListing<'info> {
  #[account(mut)]
  pub state: Box<Account<'info, State>>,

  /// CHECK: It should be the same as the one stored in the state during initialize
  #[account(
    constraint = ticket_nft_program_state.key() == state.ticket_nft_state @ ErrorCode::WrongTicketNftState
  )]
  pub ticket_nft_program_state: AccountInfo<'info>,

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

  /// CHECK: The processor will do checks like whether this account exists or if it has expired
  #[account(
    seeds = [
      b"sell_listing_reservation",
      sell_listing.key().as_ref(),
    ],
    bump,
  )]
  pub sell_listing_reservation: AccountInfo<'info>,

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

  /// CHECK: The Sale account stored in the ticket_metadata
  /// Processor will check that the ticket_metadata.sale does much this key
  #[account()]
  pub sale: AccountInfo<'info>,

  /// CHECK: THe PDA that will be sending CPI to other programs
  #[account(
    seeds = [b"market:cpi_authority", state.key().as_ref()],
    bump = state.bumps.cpi_authority,
  )]
  pub cpi_authority: AccountInfo<'info>,

  /// CHECK: The ticket metadata account.
  #[account(
    mut,
    constraint = ticket_metadata.key() == sell_listing.ticket_metadata @ ErrorCode::WrongTicketMetadata,
  )]
  pub ticket_metadata: AccountInfo<'info>,

  /// CHECK: The ticket seller
  #[account(mut)]
  pub ticket_owner: AccountInfo<'info>,

  #[account(
    mut,
    constraint = state.operators.iter().any(|m| *m == ticket_buyer.key()) @ ErrorCode::OnlyMintOperator,
  )]
  pub ticket_buyer: Signer<'info>,

  /// CHECK: this is the ticketland operator that will receive the funds if the fill listing reservation account is closed
  /// It might be same as the ticket_buyer above
  #[account(
    mut,
    constraint = state.operators.iter().any(|m| *m == operator.key()) @ ErrorCode::OnlyMintOperator,
  )]
  pub operator: AccountInfo<'info>,
    
    
  pub ticket_nft_program: Program<'info, TicketNft>,
}
