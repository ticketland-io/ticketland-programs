use anchor_lang::prelude::*;
use crate::{
  utils::program_error::ErrorCode,
  context::initialize::Initialize, 
  account_data::state::{InitBumps, Currency, MAX_CURRENCY_SUPPORT},
};

pub fn exec(
  ctx: Context<Initialize>,
  supported_currencies: Vec<Currency>,
  service_fee: u16,
  seller_fee_basis_points: u16,
) -> Result<()> {
  require!(supported_currencies.len() <= MAX_CURRENCY_SUPPORT, ErrorCode::TooManyCurrencies);
  
  let state = &mut ctx.accounts.state;

  state.bumps = InitBumps {
    event_nft_authority: *ctx.bumps.get("event_nft_authority").unwrap(),
    cpi_authority: *ctx.bumps.get("cpi_authority").unwrap(),
  };
  state.supported_currencies = supported_currencies;
  state.cpi_authority = ctx.accounts.cpi_authority.key();
  state.deployer = ctx.accounts.deployer.key();
  state.service_fee = service_fee;
  state.seller_fee_basis_points = seller_fee_basis_points;

  Ok(())
}
