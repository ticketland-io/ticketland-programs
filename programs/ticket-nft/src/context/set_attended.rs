use anchor_lang::prelude::*;
use crate::{
  utils::program_error::ErrorCode,
  account_data::{
    state::*,
    ticket_metadata::*,
  },
};

#[derive(Accounts)]
pub struct SetAttended<'info> {
  #[account()]
  pub state: Box<Account<'info, State>>,

  /// The newly created Ticket Metadata
  /// We do not apply the seeds because this IX can only be called by one of the operators
  /// which will make sure the correct ticket_metadata is passed.
  #[account(mut)]
  pub ticket_metadata: Box<Account<'info, TicketMetadata>>,

  #[account(
    mut,
    constraint = state.operators.iter().any(|u| u.key() == operator.key()) @ ErrorCode::OnlyOperator,
  )]
  pub operator: Signer<'info>,
}
