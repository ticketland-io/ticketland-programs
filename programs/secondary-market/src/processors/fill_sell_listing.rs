use anchor_lang::prelude::*;
use anchor_safe_math::SafeMath;
use anchor_spl::token::{self, Transfer};
use crate::{
  context::fill_sell_listing::*,
};

fn transfer_token<'info>(
  ctx: &Context<FillSellListing<'info>>,
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

fn transfer_funds(ctx: &Context<FillSellListing>) -> Result<()> {
  let sell_listing = &* ctx.accounts.sell_listing;
  let market = &ctx.accounts.market;
  let state = &ctx.accounts.state;

  let service_fee_amount = sell_listing.ask_price
  .safe_mul(10_000_u16.safe_sub(state.protocol_fee)? as u64)?
  .safe_div(10_000)?;

  let event_organizer_amount = sell_listing.ask_price
  .safe_mul(10_000_u16.safe_sub(market.organizer_resale_fee)? as u64)?
  .safe_div(10_000)?;

  let seller_amount = sell_listing.ask_price
  .safe_sub(event_organizer_amount)?
  .safe_sub(service_fee_amount)?;

  // transfer to treasury
  transfer_token(
    &ctx,
    ctx.accounts.ticket_buyer_ata.to_account_info().clone(),
    ctx.accounts.service_fee_ata.to_account_info().clone(),
    ctx.accounts.ticket_buyer.to_account_info().clone(),
    service_fee_amount,
  )?;

  // transfer to event organizer
  transfer_token(
    &ctx,
    ctx.accounts.ticket_buyer_ata.to_account_info().clone(),
    ctx.accounts.event_organizer_purchase_token_ata.to_account_info().clone(),
    ctx.accounts.ticket_buyer.to_account_info().clone(),
    service_fee_amount,
  )?;

  // transfer to ticket seller
  transfer_token(
    &ctx,
    ctx.accounts.ticket_buyer_ata.to_account_info().clone(),
    ctx.accounts.ticket_owner_purchase_token_ata.to_account_info().clone(),
    ctx.accounts.ticket_buyer.to_account_info().clone(),
    seller_amount,
  )?;

  Ok(())
}

pub fn exec(ctx: Context<FillSellListing>) -> Result<()> {
  transfer_funds(&ctx)?;

  Ok(())
}
