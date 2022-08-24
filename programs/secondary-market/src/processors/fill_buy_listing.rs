use anchor_lang::prelude::*;
use crate::{
  context::fill_buy_listing::*,
  processors::{
    fill_listing_common::{
      transfer_funds,
      change_ticket_ownership,
    },
  },
  acl::{
    sale_time_checks,
    sale_account,
    only_ticket_metadata_owner,
    price_cap,
    purchase_token,
    event_organizer,
  },
};

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
  purchase_token::check(
    &ctx.accounts.event,
    &ctx.accounts.purchase_token,
  )
  event_organizer::check(
    &ctx.accounts.event,
    &ctx.accounts.event_organizer,
  )
)]
pub fn exec(ctx: Context<FillBuyListing>, event_id: [u8; 32]) -> Result<()> {
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

  transfer_funds(
    ctx.accounts.buy_listing.bid_price,
    &ctx.accounts.state,
    &ctx.accounts.market,
    &ctx.accounts.token_program,
    ctx.accounts.listing_escrow_ata.to_account_info(),
    ctx.accounts.service_fee_ata.to_account_info(),
    ctx.accounts.event_organizer_purchase_token_ata.to_account_info(),
    ctx.accounts.ticket_owner_purchase_token_ata.to_account_info(),
    ctx.accounts.listing_escrow.to_account_info(),
    Some(signer_seeds),
  )?;

  change_ticket_ownership(
    &ctx.accounts.state,
    &ctx.accounts.ticket_nft_program,
    ctx.accounts.ticket_nft_program_state.to_account_info(),
    ctx.accounts.ticket_metadata.to_account_info(),
    ctx.accounts.cpi_authority.to_account_info(),
    ctx.accounts.ticket_buyer.key(),
  )?;

  // Close token account
  let cpi_accounts = CloseAccount {
      account: ctx.accounts.user_token.to_account_info(),
      destination: ctx.accounts.user.to_account_info(),
      authority: ctx.accounts.user.to_account_info(),
  };
  let cpi_program = ctx.accounts.token_program.to_account_info();
  let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
  token::close_account(cpi_ctx)?;
  
  Ok(())
}
