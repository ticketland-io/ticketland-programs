use anchor_lang::prelude::*;
use crate::{
  context::set_secondary_market::SetSecondaryMarket,
};

pub fn exec(
  ctx: Context<SetSecondaryMarket>,
  secondary_market_state: Pubkey,
  secondary_market_program: Pubkey,
) -> Result<()> {
  let state = &mut ctx.accounts.state;

  state.secondary_market_state = secondary_market_state;
  state.secondary_market_program = secondary_market_program;

  Ok(())
}
