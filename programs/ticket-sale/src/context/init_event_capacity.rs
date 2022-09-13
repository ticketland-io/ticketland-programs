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
  #[account()]
  pub state: Account<'info, State>,

  /// CHECK: The account that will hold the seats bitmap. It cannot be PDA due to space limitations.
  /// Use this constraint if you want to create an account in a previous instruction and then initialize 
  /// it in your instruction instead of using init. This is necessary for accounts that are larger than 
  /// 10 Kibibyte because those accounts cannot be created via a CPI (which is what init would do).
  #[account(
    zero,
    constraint = EventCapacity::owner() == ID @ ErrorCode::NotOwnedByThisProgram,
  )]
  pub event_capacity: Account<'info, EventCapacity>,

  #[account(
    mut,
    seeds = [b"cpi_authority", state.event_registry_state.as_ref()],
    bump = cpi_authority_bump,
    // the PDA should be owned by the Event Registry Program
    seeds::program = state.event_registry_program,
  )]
  pub event_registry_cpi_authority: Signer<'info>,
}
