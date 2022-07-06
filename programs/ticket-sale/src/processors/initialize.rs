use anchor_lang::prelude::*;
use crate::{
  context::initialize::Initialize,
};

pub fn exec(
  ctx: Context<Initialize>,
) -> Result<()> {
  let state = &mut ctx.accounts.state;

  state.event_registry_program = ctx.accounts.event_registry_program.key();
  state.event_registry_state = ctx.accounts.event_registry_state.key();
  state.event_registry_cpi_authority = ctx.accounts.event_registry_cpi_authority.key();

  Ok(())
}
