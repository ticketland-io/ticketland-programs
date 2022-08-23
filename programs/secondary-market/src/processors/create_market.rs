use anchor_lang::prelude::*;
use anchor_safe_math::SafeMath;
use common::{
  account_data::{
    serialization::deser,
  },
};
use crate::{
  context::create_market::*,
  account_data::{
    event::Event, market::MarketBump,
  },
  utils::program_error::ErrorCode,
};

fn event_account_checks(ctx: &Context<CreateMarket>) -> Result<()> {
  let event: Event = deser(ctx.accounts.event.clone())?;
  
  require!(event.event_organizer == ctx.accounts.event_organizer.key(), ErrorCode::OnlyEventOrganizer);

  Ok(())
}

pub fn exec(
  ctx: Context<CreateMarket>,
  event_id: [u8; 32],
  organizer_resale_fee: u16,
  resale_cap: u16,
) -> Result<()> {
  event_account_checks(&ctx)?;

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
