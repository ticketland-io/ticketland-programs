use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};
use crate::{
  context::cancel_buy_listing::*,
  acl::{
    purchase_token,
  },
  processors::{
    fill_listing_common::close_ata,
  },
};

fn transfer_token<'info>(
  ctx: &Context<CancelBuyListing<'info>>,
  from: AccountInfo<'info>,
  to: AccountInfo<'info>,
  authority: AccountInfo<'info>,
  amount: u64,
  signer_seeds: &[&[&[u8]]],
) -> Result<()> {
  let cpi_accounts = Transfer::<'info> {from, to, authority};
  let cpi_program = ctx.accounts.token_program.to_account_info();
  let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

  token::transfer(cpi_ctx, amount)
}

// Send aback the funds in the listing escrow ATA to the buyer
fn transfer_funds(
  ctx: &Context<CancelBuyListing>,
  bid_price: u64,
  signer_seeds: &[&[&[u8]]],
) -> Result<()> {
  transfer_token(
    &ctx,
    ctx.accounts.listing_escrow_ata.to_account_info().clone(),
    ctx.accounts.ticket_buyer_ata.to_account_info().clone(),
    ctx.accounts.listing_escrow.to_account_info(),
    bid_price,
    signer_seeds,
  )?;

  Ok(())
}

#[access_control(
  purchase_token::check(
    &ctx.accounts.event,
    &ctx.accounts.purchase_token,
  )
)]
pub fn exec(
  ctx: Context<CancelBuyListing>,
  event_id: [u8; 32],
) -> Result<()> {
  let state_key = ctx.accounts.state.key();
  let buy_listing = &ctx.accounts.buy_listing;
  let buy_listing_key = buy_listing.key();

  let seeds: &[&[u8]] = &[
    b"listing_escrow",
    state_key.as_ref(),
    &event_id,
    buy_listing_key.as_ref(),
    &[buy_listing.bumps.listing_escrow]
  ];
  let signer_seeds:&[&[&[u8]]] = &[&seeds[..]];

  // transfer funds back to the buyer
  transfer_funds(
    &ctx,
    ctx.accounts.buy_listing.bid_price,
    signer_seeds,
  )?;

  // // close the tmp listing_escrow_ata
  close_ata(
    &ctx.accounts.token_program,
    ctx.accounts.listing_escrow_ata.to_account_info(),
    ctx.accounts.ticket_buyer.to_account_info(),
    ctx.accounts.listing_escrow.to_account_info(),
    Some(signer_seeds),
  )?;

  Ok(())
}
