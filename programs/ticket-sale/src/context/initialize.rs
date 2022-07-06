use anchor_lang::prelude::*;
use std::mem::size_of;
use crate::{
  account_data::{
    state::*,
  },
};

#[derive(Accounts)]
#[instruction(event_registry_state: Pubkey)]
pub struct Initialize<'info> {
  // The state account of each instance of this program
  #[account(
    init,
    payer = deployer,
    space = 8 + size_of::<State>() + SPACE_MARGIN
  )]
  pub state: Account<'info, State>,
  
  /// CHECK: This is the Event Registry Program account
  #[account()]
  pub event_registry_program: AccountInfo<'info>,

  #[account(
    mut,
    seeds = [b"event_nft_authority", event_registry_state.as_ref()],
    // the PDA should be owned by the Event Registry Program
    seeds::program = event_registry_program.key(),
    bump
  )]
  pub event_registry_manager: Signer<'info>,

  #[account(mut)]
  pub deployer: Signer<'info>,
  
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
}
