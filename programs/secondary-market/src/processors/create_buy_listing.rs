use anchor_lang::prelude::*;
use anchor_safe_math::SafeMath;
use crate::{
  context::create_buy_listing::*,
};

pub fn exec(
  ctx: Context<CreateBuyListing>,
  market_id: [u8; 32],
  bid_price: u64,
) -> Result<()> {
  let buy_listing = &mut ctx.accounts.buy_listing;
  buy_listing.market_id = market_id;
  buy_listing.bid_price = bid_price;

  let buyer_data = &mut ctx.accounts.buyer_data;
  buyer_data.n_listing = buyer_data.n_listing.safe_add(1)?;

  Ok(())
}
