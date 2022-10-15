use anchor_lang::prelude::*;
use crate::{
  context::operator_fill_sell_listing::*,
  processors::{
    fill_listing_common::{
      change_ticket_ownership,
    },
  },
  acl::{
    sale_time_checks,
    sale_account,
    only_ticket_metadata_owner,
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
)]
pub fn exec(ctx: Context<OperatorFillSellListing>, recipient: Pubkey) -> Result<()> {
  change_ticket_ownership(
    &ctx.accounts.state,
    &ctx.accounts.ticket_nft_program,
    ctx.accounts.ticket_nft_program_state.to_account_info(),
    ctx.accounts.ticket_metadata.to_account_info(),
    ctx.accounts.cpi_authority.to_account_info(),
    recipient,
  )?;
  
  Ok(())
}
