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
  
  /// CHECK: The authority of all minted NFTs
  #[account(
    init,
    payer = deployer,
    space = 0,
    seeds = [b"nft_authority", state.key().as_ref()],
    bump,
  )]
  pub nft_authority: AccountInfo<'info>,


  #[account(mut)]
  pub deployer: Signer<'info>,
  
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
}
