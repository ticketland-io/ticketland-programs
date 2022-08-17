use anchor_lang::prelude::*;
use crate::{
  context::initialize::Initialize,
};

pub fn exec(
  ctx: Context<Initialize>,
  ticket_sale_state: Pubkey,
  ticket_sale_program: Pubkey,
  protocol_fee: u16,
) -> Result<()> {
  let state = &mut ctx.accounts.state;

  state.ticket_sale_state = ticket_sale_state;
  state.ticket_sale_program = ticket_sale_program;
  state.deployer = ctx.accounts.deployer.key();
  state.protocol_fee = protocol_fee;

  Ok(())
}
