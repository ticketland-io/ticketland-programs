use anchor_lang::prelude::*;
use crate::{
  context::initialize::Initialize,
  account_data::{
    state::*,
  },
};

pub fn exec(
  ctx: Context<Initialize>,
  ticket_sale_state: Pubkey,
  ticket_sale_program: Pubkey,
) -> Result<()> {
  let state = &mut ctx.accounts.state;

  state.bumps = InitBumps {
    nft_authority: *ctx.bumps.get("nft_authority").unwrap(),
  };
  state.nft_authority = ctx.accounts.nft_authority.key();
  state.ticket_sale_program = ticket_sale_program;
  state.ticket_sale_state = ticket_sale_state;
  state.deployer = ctx.accounts.deployer.key();

  Ok(())
}
