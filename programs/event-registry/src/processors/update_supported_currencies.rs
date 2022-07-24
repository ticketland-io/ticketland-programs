use anchor_lang::prelude::*;
use common::{
  state::currency::*,
};
use crate::{
  utils::program_error::ErrorCode,
  context::update_supported_currencies::UpdateSupportedCurrencies, 
  account_data::state::{MAX_CURRENCY_SUPPORT},
};

pub fn exec(
  ctx: Context<UpdateSupportedCurrencies>,
  supported_currencies: Vec<Currency>,
) -> Result<()> {
  require!(supported_currencies.len() <= MAX_CURRENCY_SUPPORT, ErrorCode::TooManyCurrencies);
  let state = &mut ctx.accounts.state;

  state.supported_currencies = supported_currencies;

  Ok(())
}
