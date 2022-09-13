use anchor_lang::prelude::*;
use std::mem::size_of;
use crate::{
  account_data::{
    state::*,
    sale::Sale,
  },
};

#[derive(Accounts)]
#[instruction(ticket_type_index: u8, event_id: [u8; 32])]
pub struct CreateSale<'info> {
  #[account()]
  pub state: Account<'info, State>,

  /// CHECK: The Event account.
  /// Constraints will be checked in the processor fn
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

  /// The newly created Sale 
  #[account(
    init,
    payer = event_organizer,
    space = 8 + size_of::<Sale>(),
    seeds = [
      b"sale",
      state.key().as_ref(),
      ticket_type_index.to_string().as_ref(),
      &event_id
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
