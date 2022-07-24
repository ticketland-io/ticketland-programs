use anchor_lang::prelude::*;
use crate::{
  account_data::{
    state::*,
  },
  utils::program_error::ErrorCode,
};

#[derive(Accounts)]
pub struct UpdateSupportedCurrencies<'info> {
  #[account()]
  pub state: Account<'info, State>,

  #[account(
    mut,
    constraint = deployer.key() == state.deployer @ ErrorCode::OnlyDeployer,
  )]
  pub deployer: Signer<'info>,
}
