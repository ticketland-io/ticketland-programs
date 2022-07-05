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
    space = 8 + size_of::<State>() + size_of::<Currency>() * MAX_CURRENCY_SUPPORT + SPACE_MARGIN
  )]
  pub state: Account<'info, State>,

  /// CHECK: The authority of the event nfts
  #[account(
    init,
    payer = deployer,
    space = 0,
    seeds = [b"event_nft_authority", state.key().as_ref()],
    bump,
  )]
  pub event_nft_authority: AccountInfo<'info>,

  #[account(mut)]
  pub deployer: Signer<'info>,

  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
}
