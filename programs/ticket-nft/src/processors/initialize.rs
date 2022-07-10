use anchor_lang::prelude::*;
use crate::{
  context::initialize::Initialize,
  account_data::{
    state::*,
  },
};

pub fn exec(
  ctx: Context<Initialize>,
) -> Result<()> {
  let state = &mut ctx.accounts.state;

  state.bumps = InitBumps {
    nft_authority: *ctx.bumps.get("nft_authority").unwrap(),
  };
  state.nft_authority = ctx.accounts.nft_authority.key();
  state.ticket_sale_program = ctx.accounts.ticket_sale_program.key();
  state.ticket_sale_state = ctx.accounts.ticket_sale_state.key();
  state.deployer = ctx.accounts.deployer.key();

  Ok(())
}
