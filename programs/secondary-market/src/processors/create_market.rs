use anchor_lang::prelude::*;
use anchor_safe_math::SafeMath;
use crate::{
  context::create_market::*,
  account_data::{
    market::*,
  },
  acl::{
    event_organizer,
  }, 
};

#[access_control(
  event_organizer::check(
    &ctx.accounts.event,
    &ctx.accounts.event_organizer,
  )
)]
pub fn exec(
  ctx: Context<CreateMarket>,
  event_id: [u8; 32],
  organizer_resale_fee: u16,
  resale_cap: u16,
) -> Result<()> {
  let state = &mut ctx.accounts.state;
  state.n_markets =  state.n_markets.safe_add(1)?;

  let market = &mut ctx.accounts.market;
  
  market.bumps = MarketBump {
    market: *ctx.bumps.get("market").unwrap(),
  };
  market.event_id = event_id;
  market.organizer_resale_fee = organizer_resale_fee;
  market.resale_cap = resale_cap;
  
  Ok(())
}
