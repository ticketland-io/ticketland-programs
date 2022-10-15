use anchor_lang::prelude::*;
use crate::{
  context::initialize::Initialize,
  account_data::state::InitBumps,
};

pub fn exec(
  ctx: Context<Initialize>,
  treasury: Pubkey,
  event_registry_state: Pubkey,
  event_registry_program: Pubkey,
  mint_operators: Vec<Pubkey>,
) -> Result<()> {
  let state = &mut ctx.accounts.state;

  state.bumps = InitBumps {
    cpi_authority: *ctx.bumps.get("cpi_authority").unwrap(),
  };

  state.total_sold = 0;
  state.event_registry_program = event_registry_program;
  state.event_registry_state = event_registry_state;
  state.treasury = treasury;
  state.deployer = ctx.accounts.deployer.key();
  state.cpi_authority = ctx.accounts.cpi_authority.key();
  state.mint_operators = mint_operators;

  Ok(())
}
