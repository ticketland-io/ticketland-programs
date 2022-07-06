use anchor_lang::prelude::*;
use std::mem::size_of;
use crate::{
  account_data::{
    state::*,
    sale::{Sale, SPACE_MARGIN},
  },
};

#[derive(Accounts)]
#[instruction(ticket_type_index: u8, cpi_authority_bump: u8, event_id: u64, event_registry_program: Pubkey)]
pub struct CreateSale<'info> {
  #[account(mut)]
  pub state: Account<'info, State>,

  /// CHECK: The state account of the event registry program
  #[account()]
  pub event_registry_state: AccountInfo<'info>,

  /// The newly created Sale 
  #[account(
    init,
    payer = event_organizer,
    space = 8 + size_of::<Sale>() + SPACE_MARGIN,
    seeds = [
      b"sale", state.key().as_ref(),
      ticket_type_index.to_string().as_ref(),
      &event_id.to_string().as_ref()
    ],
    bump
  )]
  pub sale: Account<'info, Sale>,

  #[account(
    mut,
    seeds = [b"cpi_authority", event_registry_state.key().as_ref()],
    // the PDA should be owned by the Event Registry Program
    seeds::program = event_registry_program,
    bump = cpi_authority_bump,
    constraint = event_registry_cpi_authority.key() == state.event_registry_cpi_authority.key(),
  )]
  pub event_registry_cpi_authority: Signer<'info>,

  /// This is the user that calls the create event in the event registry program
  #[account(mut)]
  pub event_organizer: Signer<'info>,

  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
}
