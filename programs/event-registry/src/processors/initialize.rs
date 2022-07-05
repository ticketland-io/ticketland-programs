use anchor_lang::prelude::*;
use crate::{
  context::initialize::Initialize, 
  account_data::state::{InitBumps, Currency},
};

pub fn exec(
  ctx: Context<Initialize>,
  supported_currencies: Vec<Currency>,
  service_fee: u16,
) -> Result<()> {
  let state = &mut ctx.accounts.state;

  state.bumps = InitBumps {
    fund_manager: *ctx.bumps.get("fund_manager").unwrap(),
    event_nft_authority: *ctx.bumps.get("event_nft_authority").unwrap(),
  };
  state.supported_currencies = supported_currencies;
  state.deployer = ctx.accounts.deployer.key();
  state.service_fee = service_fee;

  Ok(())
}
