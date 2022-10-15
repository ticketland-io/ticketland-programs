use anchor_lang::prelude::*;
use crate::{
  context::fill_sell_listing::*,
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
pub fn exec(ctx: Context<FillSellListing>) -> Result<()> {
  Ok(())
}
