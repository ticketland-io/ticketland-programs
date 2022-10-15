use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};
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
  context::fixed_price_purchase::*,
  account_data::{
    event::Event,
  },
  utils::program_error::ErrorCode,
};
use super::common_purchase::{
  post_checks,
};

fn transfer_token<'info>(
  ctx: &Context<FixedPricePurchase<'info>>,
  from: AccountInfo<'info>,
  to: AccountInfo<'info>,
  authority: AccountInfo<'info>,
  amount: u64,
) -> Result<()> {
  let cpi_accounts = Transfer::<'info> {from, to, authority};
  let cpi_program = ctx.accounts.token_program.to_account_info();
  let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

  token::transfer(cpi_ctx, amount)
}

/// Transfer the purchase funds to event organizer and our treasury
fn transfer_funds(ctx: &Context<FixedPricePurchase>, event: &Event) -> Result<u64> {
  let ticket_type = &ctx.accounts.sale.ticket_type;
  let amount = if let SaleType::FixedPrice {amount} = ticket_type.sale_type {
    amount
  } else {
    // This should never happen since we already have the same check in the FixedPricePurchase context
    return Err(ErrorCode::UnexpectedSaleAccount.into());
  };
  let (event_organizer_amount, service_fee_amount) = event.currency.calc_fee(amount)?;

  // send to treasury
  if service_fee_amount > 0 {
    transfer_token(
      &ctx,
      ctx.accounts.ticket_buyer_ata.to_account_info().clone(),
      ctx.accounts.service_fee_ata.to_account_info().clone(),
      ctx.accounts.ticket_buyer.to_account_info().clone(),
      service_fee_amount,
    )?;
  }

  // send to event organizer
  if event_organizer_amount > 0 {
    transfer_token(
      &ctx,
      ctx.accounts.ticket_buyer_ata.to_account_info().clone(),
      ctx.accounts.event_organizer_purchase_token_ata.to_account_info().clone(),
      ctx.accounts.ticket_buyer.to_account_info().clone(),
      event_organizer_amount,
    )?;
  }

  Ok(amount)
}

pub fn exec(
  ctx: Context<FixedPricePurchase>,
  seat_index: u32,
  seat_name: String,
) -> Result<()> {
  let event: Event = deser(ctx.accounts.event.clone())?;
  require!(event.currency.mint_account == ctx.accounts.purchase_token.key(), ErrorCode::UnsupportedPurchaseToken);

  expand_pre_checks!(ctx, event, seat_index);
  let price_sold = transfer_funds(&ctx, &event)?;
  expand_mint_ticket!(ctx, price_sold, seat_index, seat_name);
  post_checks(&mut ctx.accounts.state, &mut ctx.accounts.event_capacity, seat_index)?;

  Ok(())
}
