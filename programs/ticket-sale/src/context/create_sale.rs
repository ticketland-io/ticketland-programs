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
#[instruction(sale_type_index: usize)]
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
      sale_type_index.to_string().as_ref(),
      &event_registry_state.n_events.to_string().as_ref()
    ],
    bump
  )]
  pub sale: Account<'info, Sale>,

  /// This is the user that calls the create event in the event registry program
  #[account(mut)]
  pub event_organizer: Signer<'info>,

  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
}
