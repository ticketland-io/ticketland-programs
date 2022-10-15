use anchor_lang::prelude::*;
use common::{
  account_data::{
    serialization::deser,
  },
  state::{
    sale_type::*,
  },
};
use crate::{
  expand_pre_checks,
  expand_mint_ticket,
  account_data::{
    event::Event,
  },
  utils::program_error::ErrorCode,
  context::operator_purchase::OperatorPurchase,
};
use super::common_purchase::{
  post_checks,
};

pub fn exec(
  ctx: Context<OperatorPurchase>,
  seat_index: u32,
  seat_name: String,
  recipient: Pubkey,
) -> Result<()> {
  let event: Event = deser(ctx.accounts.event.clone())?;
  let ticket_type = &ctx.accounts.sale.ticket_type;
  let price_sold = if let SaleType::FixedPrice {amount} = ticket_type.sale_type {
    amount
  } else {
    return Err(ErrorCode::UnexpectedSaleAccount.into());
  };

  expand_pre_checks!(ctx, event, seat_index);
  expand_mint_ticket!(ctx, recipient, price_sold, seat_index, seat_name);
  post_checks(&mut ctx.accounts.state, &mut ctx.accounts.event_capacity, seat_index)?;

  Ok(())
}
