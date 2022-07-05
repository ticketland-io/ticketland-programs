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
    // 200 bytes max length of the base_uri
    // MAX Creators 5
    space = 8 + size_of::<State>() + size_of::<Pubkey>() * MAX_CURRENCY_SUPPORT + SPACE_MARGIN
  )]
  pub state: Account<'info, State>,

  /// CHECK: The PDA that will be the authority to handle all deposits
  #[account(
    init,
    payer = deployer,
    space = 0,
    seeds = [b"fund_manager", state.key().as_ref()],
    bump,
  )]
  pub fund_manager: AccountInfo<'info>,

  #[account(mut)]
  pub deployer: Signer<'info>,

  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
}
