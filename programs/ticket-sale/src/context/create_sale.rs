use anchor_lang::prelude::*;
use std::mem::size_of;
use event_registry::{
  account_data::{
    state::{State as EventRegistryState},
  }
};
use crate::{
  account_data::{
    state::*,
    sale::{Sale, SPACE_MARGIN},
  },
};

#[derive(Accounts)]
#[instruction(ticket_type_index: u8)]
pub struct CreateSale<'info> {
  #[account(mut)]
  pub state: Account<'info, State>,

  /// CHECK: The state account of the event registry program
  #[account()]
  pub event_registry_state: Account<'info, EventRegistryState>,

  /// The newly created Sale 
  #[account(
    init,
    payer = event_organizer,
    space = 8 + size_of::<Sale>() + SPACE_MARGIN,
    seeds = [
      b"sale", state.key().as_ref(),
      ticket_type_index.to_string().as_ref(),
      &event_registry_state.n_events.to_string().as_ref()
    ],
    bump
  )]
  pub sale: Account<'info, Sale>,

  /// CHECK: This is the Event Registry Program account
  #[account()]
  pub event_registry_program: AccountInfo<'info>,

  #[account(
    mut,
    seeds = [b"cpi_authority", event_registry_state.key().as_ref()],
    // the PDA should be owned by the Event Registry Program
    seeds::program = event_registry_program.key(),
    bump = event_registry_state.bumps.cpi_authority,
    constraint = event_registry_cpi_authority.key() == state.event_registry_cpi_authority.key(),
  )]
  pub event_registry_cpi_authority: Signer<'info>,

  /// This is the user that calls the create event in the event registry program
  #[account(mut)]
  pub event_organizer: Signer<'info>,

  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
}
