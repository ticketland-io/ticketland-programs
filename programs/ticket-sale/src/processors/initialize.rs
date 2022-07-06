use anchor_lang::prelude::*;
use crate::{
  context::initialize::Initialize,
};


pub fn exec(
  ctx: Context<Initialize>,
  event_registry_state: Pubkey,
) -> Result<()> {
  let state = &mut ctx.accounts.state;

  state.event_registry_program = ctx.accounts.event_registry_program.key();
  state.event_registry_state = event_registry_state;
  state.event_registry_manager = ctx.accounts.event_registry_manager.key();

  Ok(())
}
