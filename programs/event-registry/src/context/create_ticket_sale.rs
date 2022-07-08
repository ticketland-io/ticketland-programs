use anchor_lang::prelude::*;
use ticket_sale::{
  program::TicketSale,
};
use crate::{
  utils::{
    program_error::ErrorCode,
  },
  account_data::{
    state::*,
    event::*,
  },
};

#[derive(Accounts)]
#[instruction(ticket_type_index: usize, event_id: u64,)]
pub struct CreateTicketSale<'info> {
  #[account(mut)]
  pub state: Account<'info, State>,

  // The newly created event 
  #[account(
    seeds = [b"event", state.key().as_ref(), &event_id.to_string().as_ref()],
    bump = event.bumps.event,
  )]
  pub event: Account<'info, Event>,

  /// CHECK: The state of the ticker program sale
  #[account(
    mut,
    constraint = ticket_sale_program_state.owner.key() == ticket_sale_program.key() @ ErrorCode::WrongTicketSaleProgramStateAccount
  )]
  pub ticket_sale_program_state: AccountInfo<'info>,

  /// CHECK: The state of the ticket sale that will be created in the ticket sale program.
  /// Note that we use AccountInfo which means that there will be no checks if the account indeed has the Sale 
  /// data. However, since this is will further be passed as a CPI account to the Ticket Sale, the latter will
  /// do all the checks needed.
  #[account(
    mut,
    seeds = [
      b"sale",
      ticket_sale_program_state.key().as_ref(),
      ticket_type_index.to_string().as_ref(),
      &event.id.to_string().as_ref()
    ],
    bump,
    seeds::program = ticket_sale_program.key(),
    constraint = ticket_type_index <= event.ticket_types.len() @ ErrorCode::InvalidTicketTypeIndex
  )]
  pub ticket_sale_state: AccountInfo<'info>,

  /// CHECK: THe PDA that will be sending CPI to other programs i.e. TicketSale Program
  #[account(
    mut,
    seeds = [b"cpi_authority", state.key().as_ref()],
    bump = state.bumps.cpi_authority,
  )]
  pub cpi_authority: AccountInfo<'info>,

  /// This is the user that created the event earlier
  #[account(
    mut,
    constraint = event_organizer.key() == event.event_organizer @ ErrorCode::OnlyEventOrganizer
  )]
  pub event_organizer: Signer<'info>,

  pub ticket_sale_program: Program<'info, TicketSale>,
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
}
