use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};
use anchor_safe_math::SafeMath;
use common::{
  utils::bitmap,
  account_data::{
    serialization::deser,
  },
  state::{
    sale_type::*,
  },
};
use crate::{
  context::create_market::*,
  account_data::{
    event::Event,
  },
  utils::program_error::ErrorCode,
};

fn event_account_checks(ctx: &Context<CreateMarket>, event_id: [u8; 32]) -> Result<()> {
  let event: Event = deser(ctx.accounts.event.clone())?;
  
  require!(event.id == event_id, ErrorCode::WrongEventAccount);
  require!(event.event_organizer == ctx.accounts.event_organizer.key(), ErrorCode::WrongEventOrganizer);

  Ok(())
}

pub fn exec(
  ctx: Context<CreateMarket>,
  market_id: [u8; 32],
  event_id: [u8; 32],
  organizer_resale_fee: u16,
  resale_cap: u16,
) -> Result<()> {
  event_account_checks(&ctx, event_id)?;

  let state = &mut ctx.accounts.state;
  state.nMarkets =  state.nMarkets.safe_add(1)?;

  let market = &mut ctx.accounts.market;
  market.id = market_id;
  market.event_id = event_id;
  market.organizer_resale_fee = organizer_resale_fee;
  market.resale_cap = resale_cap;

  todo!()
}
