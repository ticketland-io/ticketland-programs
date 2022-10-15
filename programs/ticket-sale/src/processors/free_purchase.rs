use anchor_lang::prelude::*;
use common::{
  account_data::{
    serialization::deser,
  },
};
use crate::{
  expand_pre_checks,
  expand_mint_ticket,
  account_data::{
    event::Event,
  },
  context::free_purchase::FreePurchase,
};
use super::common_purchase::{
  post_checks,
};

pub fn exec(
  ctx: Context<FreePurchase>,
  seat_index: u32,
  seat_name: String,
) -> Result<()> {
  let event: Event = deser(ctx.accounts.event.clone())?;

  expand_pre_checks!(ctx, event, seat_index);
  expand_mint_ticket!(ctx, 0_u64, seat_index, seat_name);
  post_checks(&mut ctx.accounts.state, &mut ctx.accounts.event_capacity, seat_index)?;

  Ok(())
}
