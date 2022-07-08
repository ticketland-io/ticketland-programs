use anchor_lang::prelude::*;
use std::mem::size_of;
use crate::{
  account_data::{
    state::*,
  },
};

#[derive(Accounts)]
pub struct Initialize<'info> {
  // The state account of each instance of this program
  #[account(
    init,
    payer = deployer,
    space = 8 + size_of::<State>() + SPACE_MARGIN
  )]
  pub state: Account<'info, State>,
  
  /// CHECK: The state account of the event registry program
  #[account()]
  pub event_registry_state: AccountInfo<'info>,

  /// CHECK: This is the Event Registry Program account
  #[account()]
  pub event_registry_program: AccountInfo<'info>,

  #[account(mut)]
  pub deployer: Signer<'info>,
  
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
}
