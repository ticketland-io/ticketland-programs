use anchor_lang::prelude::*;
use crate::{
  context::initialize::Initialize, 
  account_data::state::{InitBumps, Currency},
};

pub fn exec(
  ctx: Context<Initialize>,
  supported_currencies: Vec<Currency>,
  service_fee: u16,
  seller_fee_basis_points: u16,
) -> Result<()> {
  let state = &mut ctx.accounts.state;

  state.bumps = InitBumps {
    event_nft_authority: *ctx.bumps.get("event_nft_authority").unwrap(),
  };
  state.supported_currencies = supported_currencies;
  state.deployer = ctx.accounts.deployer.key();
  state.service_fee = service_fee;
  state.seller_fee_basis_points = seller_fee_basis_points;

  Ok(())
}
