use anchor_lang::prelude::*;
use anchor_safe_math::SafeMath;
use anchor_spl::token::{self, Transfer};
use crate::{
  context::create_buy_listing::*,
  account_data::{
    buy_listing::*,
  },
};

fn transfer_token<'info>(
  ctx: &Context<CreateBuyListing<'info>>,
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

// Lock the funds in the listing escrow ATA so when this buy_listing is filled, the seller of the ticket nft
// receives these funds.
fn transfer_funds(ctx: &Context<CreateBuyListing>, bid_price: u64) -> Result<()> {
  transfer_token(
    &ctx,
    ctx.accounts.ticket_buyer_ata.to_account_info().clone(),
    ctx.accounts.listing_escrow_ata.to_account_info().clone(),
    ctx.accounts.ticket_buyer.to_account_info().clone(),
    bid_price,
  )?;

  Ok(())
}

pub fn exec(
  ctx: Context<CreateBuyListing>,
  bid_price: u64,
) -> Result<()> {
  transfer_funds(&ctx, bid_price)?;

  let buy_listing = &mut ctx.accounts.buy_listing;

  buy_listing.bumps = BuyListingBumps {
    listing_escrow: *ctx.bumps.get("listing_escrow").unwrap(),
  };
  buy_listing.bid_price = bid_price;

  let buyer_data = &mut ctx.accounts.buyer_data;
  buyer_data.n_listing = buyer_data.n_listing.safe_add(1)?;

  Ok(())
}
