use anchor_lang::prelude::*;
use crate::{
  account_data::{
    state::*,
    ticket_metadata::*,
  },
};

#[derive(Accounts)]
pub struct Transfer<'info> {
  #[account()]
  pub state: Box<Account<'info, State>>,

  /// The newly created Ticket Metadata 
  /// We do not apply the seeds because this IX can only be called by the secondary_market_cpi_authority
  /// which will make sure the correct ticket_metadata is passed.
  #[account(mut)]
  pub ticket_metadata: Box<Account<'info, TicketMetadata>>,

  /// CHECK: THe PDA that will be sending CPI to other programs i.e. TicketSale Program
  #[account(
    seeds = [b"market:cpi_authority", state.secondary_market_state.as_ref()],
    bump,
    seeds::program = state.secondary_market_program,
  )]
  pub secondary_market_cpi_authority: AccountInfo<'info>,
}
