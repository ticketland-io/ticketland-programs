use anchor_lang::prelude::*;
use common::{
  account_data::event_registry_state::{InitBumps, Currency},
};
use crate::{
  utils::program_error::ErrorCode,
  context::initialize::Initialize, 
  account_data::state::{MAX_CURRENCY_SUPPORT},
};

pub fn exec(
  ctx: Context<Initialize>,
  supported_currencies: Vec<Currency>,
  service_fee: u16,
  seller_fee_basis_points: u16,
) -> Result<()> {
  require!(supported_currencies.len() <= MAX_CURRENCY_SUPPORT, ErrorCode::TooManyCurrencies);
  
  let state = &mut ctx.accounts.state;

  state.0.bumps = InitBumps {
    event_nft_authority: *ctx.bumps.get("event_nft_authority").unwrap(),
    cpi_authority: *ctx.bumps.get("cpi_authority").unwrap(),
  };
  state.0.supported_currencies = supported_currencies;
  state.0.deployer = ctx.accounts.deployer.key();
  state.0.service_fee = service_fee;
  state.0.seller_fee_basis_points = seller_fee_basis_points;

  Ok(())
}
