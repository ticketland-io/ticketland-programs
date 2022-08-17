use anchor_lang::prelude::*;
use crate::{
  context::initialize::Initialize,
};

pub fn exec(
  ctx: Context<Initialize>,
  protocol_fee: u16,
) -> Result<()> {
  let state = &mut ctx.accounts.state;

  state.protocol_fee = protocol_fee;
  state.deployer = ctx.accounts.deployer.key();

  Ok(())
}
