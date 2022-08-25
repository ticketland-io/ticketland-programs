use anchor_lang::prelude::*;
use crate::{
  context::fill_sell_listing::*,
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
    purchase_token,
    event_organizer,
  },
};

#[access_control(
  sale_time_checks::check(&ctx.accounts.sale)
  sale_account::check(
    &ctx.accounts.ticket_metadata,
    &ctx.accounts.sale
  )
  only_ticket_metadata_owner::check(
    &ctx.accounts.ticket_metadata,
    &ctx.accounts.ticket_owner,
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
pub fn exec(ctx: Context<FillSellListing>) -> Result<()> {
  transfer_funds(
    ctx.accounts.sell_listing.ask_price,
    &ctx.accounts.state,
    &ctx.accounts.market,
    &ctx.accounts.token_program,
    ctx.accounts.ticket_buyer_ata.to_account_info(),
    ctx.accounts.service_fee_ata.to_account_info(),
    ctx.accounts.event_organizer_purchase_token_ata.to_account_info(),
    ctx.accounts.ticket_owner_purchase_token_ata.to_account_info(),
    ctx.accounts.ticket_buyer.to_account_info(),
    None,
  )?;

  change_ticket_ownership(
    &ctx.accounts.state,
    &ctx.accounts.ticket_nft_program,
    ctx.accounts.ticket_nft_program_state.to_account_info(),
    ctx.accounts.ticket_metadata.to_account_info(),
    ctx.accounts.cpi_authority.to_account_info(),
    ctx.accounts.ticket_buyer.key(),
  )?;
  
  Ok(())
}
