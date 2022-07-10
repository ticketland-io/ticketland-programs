use anchor_lang::prelude::*;
use std::mem::size_of;
use crate::{
  account_data::{
    state::*,
    sale::{Sale, SPACE_MARGIN},
  },
};

#[derive(Accounts)]
#[instruction(cpi_authority_bump: u8, ticket_type_index: usize, event_id: u64)]
pub struct CreateSale<'info> {
  #[account()]
  pub state: Account<'info, State>,

  /// The newly created Sale 
  #[account(
    init,
    payer = event_organizer,
    space = 8 + size_of::<Sale>() + SPACE_MARGIN,
    seeds = [
      b"sale",
      state.key().as_ref(),
      ticket_type_index.to_string().as_ref(),
      &event_id.to_string().as_ref()
    ],
    bump
  )]
  pub sale: Account<'info, Sale>,

  #[account(
    mut,
    seeds = [b"cpi_authority", state.event_registry_state.as_ref()],
    bump = cpi_authority_bump,
    // the PDA should be owned by the Event Registry Program
    seeds::program = state.event_registry_program,
  )]
  pub event_registry_cpi_authority: Signer<'info>,

  /// This is the user that calls the create event in the event registry program
  #[account(mut)]
  pub event_organizer: Signer<'info>,

  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
}
