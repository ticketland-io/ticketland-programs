use anchor_lang::prelude::*;
use crate::{
  account_data::{
    state::*,
  },
  utils::{
    program_error::ErrorCode,
  }
};

#[derive(Accounts)]
pub struct SetSecondaryMarket<'info> {
  // The state account of each instance of this program
  #[account(mut)]
  pub state: Account<'info, State>,
  
  #[account(
    mut,
    constraint = state.deployer == deployer.key() @ ErrorCode::OnlyDeployer,
  )]
  pub deployer: Signer<'info>,
}
