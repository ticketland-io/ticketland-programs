use anchor_lang::prelude::*;
use anchor_safe_math::SafeMath;
use anchor_spl::token::{self, Transfer};
use ticket_nft::{
  account_data::{
    ticket_metadata::*,
  },
};
use crate::{
  context::fill_buy_listing::*,
  acl::{
    sale_time_checks,
    sale_account,
    only_ticket_metadata_owner,
    price_cap,
  },
};

fn transfer_token<'info>(
  ctx: &Context<FillBuyListing<'info>>,
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

fn transfer_funds(ctx: &Context<FillBuyListing>) -> Result<()> {
  let buy_listing = &* ctx.accounts.buy_listing;
  let market = &ctx.accounts.market;
  let state = &ctx.accounts.state;

  let service_fee_amount = buy_listing.bid_price
  .safe_mul(10_000_u16.safe_sub(state.protocol_fee)? as u64)?
  .safe_div(10_000)?;

  let event_organizer_amount = buy_listing.bid_price
  .safe_mul(10_000_u16.safe_sub(market.organizer_resale_fee)? as u64)?
  .safe_div(10_000)?;

  let seller_amount = buy_listing.bid_price
  .safe_sub(event_organizer_amount)?
  .safe_sub(service_fee_amount)?;

  // transfer to treasury
  if service_fee_amount > 0 {
    transfer_token(
      &ctx,
      ctx.accounts.listing_escrow_ata.to_account_info().clone(),
      ctx.accounts.service_fee_ata.to_account_info().clone(),
      ctx.accounts.listing_escrow.to_account_info().clone(),
      service_fee_amount,
    )?;
  }

  // transfer to event organizer
  if event_organizer_amount > 0 {
    transfer_token(
      &ctx,
      ctx.accounts.listing_escrow_ata.to_account_info().clone(),
      ctx.accounts.event_organizer_purchase_token_ata.to_account_info().clone(),
      ctx.accounts.listing_escrow.to_account_info().clone(),
      event_organizer_amount,
    )?;
  }

  // transfer to ticket seller
  if seller_amount > 0 {
    transfer_token(
      &ctx,
      ctx.accounts.listing_escrow_ata.to_account_info().clone(),
      ctx.accounts.ticket_owner_purchase_token_ata.to_account_info().clone(),
      ctx.accounts.listing_escrow.to_account_info().clone(),
      seller_amount,
    )?;
  }

  Ok(())
}

fn change_ticket_ownership(ctx: &Context<FillBuyListing>) -> Result<()> {
  let cpi_program = ctx.accounts.ticket_nft_program.to_account_info();
  let cpi_accounts = ticket_nft::cpi::accounts::Transfer {
    state: ctx.accounts.ticket_nft_program_state.to_account_info(),
    ticket_metadata: ctx.accounts.ticket_metadata.to_account_info(),
    secondary_market_cpi_authority:  ctx.accounts.cpi_authority.to_account_info(),
  };

  let state = &ctx.accounts.state;
  let state_key = state.key();
  let seeds: &[&[u8]] = &[
    b"market:cpi_authority", state_key.as_ref(),
    &[state.bumps.cpi_authority]
  ];
  let signer_seeds:&[&[&[u8]]] = &[&seeds[..]];
  let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

  ticket_nft::cpi::transfer(
    cpi_ctx,
    ctx.accounts.ticket_buyer.key(),
  )?;

  Ok(())
}

#[access_control(
  sale_time_checks::check(&ctx.accounts.sale)
  sale_account::check(
    &ctx.accounts.ticket_metadata,
    &ctx.accounts.sale,
  )
  only_ticket_metadata_owner::check(
    &ctx.accounts.ticket_metadata,
    &ctx.accounts.ticket_owner,
  )
  price_cap::check(
    &ctx.accounts.ticket_metadata,
    ctx.accounts.market.resale_cap,
    ctx.accounts.buy_listing.bid_price,
  )
)]
pub fn exec(ctx: Context<FillBuyListing>) -> Result<()> {
  transfer_funds(&ctx)?;
  change_ticket_ownership(&ctx)?;

  Ok(())
}
