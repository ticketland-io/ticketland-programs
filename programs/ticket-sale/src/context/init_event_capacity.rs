use anchor_lang::prelude::*;
use crate::{
  ID,
  account_data::{
    state::*,
    event_capacity::EventCapacity,
  },
  utils::program_error::ErrorCode,
};
#[derive(Accounts)]
#[instruction(cpi_authority_bump: u8)]
pub struct InitEventCapacity<'info> {
  #[account(mut)]
  pub state: Account<'info, State>,

  /// CHECK: The account that will hold the seats bitmap. It cannot be PDA due to space limitations.
  #[account(
    zero,
    constraint = EventCapacity::owner() == ID @ ErrorCode::NotOwnedByThisProgram,
  )]
  pub event_capacity: AccountLoader<'info, EventCapacity>,

  #[account(
    mut,
    seeds = [b"cpi_authority", state.event_registry_state.as_ref()],
    bump = cpi_authority_bump,
    // the PDA should be owned by the Event Registry Program
    seeds::program = state.event_registry_program,
  )]
  pub event_registry_cpi_authority: Signer<'info>,
}
