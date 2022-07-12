use anchor_lang::prelude::*;
use crate::{
  context::initialize::Initialize,
  account_data::state::InitBumps,
};

pub fn exec(
  ctx: Context<Initialize>,
  treasury: Pubkey,
) -> Result<()> {
  let state = &mut ctx.accounts.state;

  state.bumps = InitBumps {
    cpi_authority: *ctx.bumps.get("cpi_authority").unwrap(),
  };

  state.total_sold = 0;
  state.event_registry_program = ctx.accounts.event_registry_program.key();
  state.event_registry_state = ctx.accounts.event_registry_state.key();
  state.treasury = treasury;
  state.cpi_authority = ctx.accounts.cpi_authority.key();
  state.deployer = ctx.accounts.deployer.key();

  Ok(())
}
