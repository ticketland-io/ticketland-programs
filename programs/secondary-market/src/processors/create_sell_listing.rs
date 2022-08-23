use anchor_lang::prelude::*;
use common::{
  account_data::{
    serialization::deser,
  },
};
use ticket_nft::{
  account_data::{
    ticket_metadata::*,
  },
};
use crate::{
  context::create_sell_listing::*,
  acl::{
    sale_time_checks,
    sale_account,
    price_cap,
    purchase_token,
  },
  utils::program_error::ErrorCode,
};

/// Check that the as ticket metadata account has the same event id as the one that was passed as param.
/// Additionally, check that the owner of the ticket metadata (and thus the owner of the ticket nft) is the signer
/// of this tx
fn ticket_metadata_account_checks(
  ctx: &Context<CreateSellListing>,
  event_id: [u8; 32],
) -> Result<()> {
  let ticket_metadata: TicketMetadata = deser(ctx.accounts.ticket_metadata.clone())?;
  
  require!(ticket_metadata.event_id == event_id, ErrorCode::EventIdDoesNotMatch);
  require!(ticket_metadata.owner == ctx.accounts.ticket_owner.key(), ErrorCode::OnlyTicketOwner);

  Ok(())
}

#[access_control(
  sale_time_checks::check(&ctx.accounts.sale)
  sale_account::check(
    &ctx.accounts.ticket_metadata,
    &ctx.accounts.sale
  )
  price_cap::check(
    &ctx.accounts.ticket_metadata,
    ctx.accounts.market.resale_cap,
    ask_price,
  )
  purchase_token::check(
    &ctx.accounts.event,
    &ctx.accounts.purchase_token,
  )
)]
pub fn exec(
  ctx: Context<CreateSellListing>,
  event_id: [u8; 32],
  ask_price: u64,
) -> Result<()> {
  ticket_metadata_account_checks(&ctx, event_id)?;

  let sell_listing = &mut ctx.accounts.sell_listing;
  sell_listing.ask_price = ask_price;
  sell_listing.ticket_metadata = ctx.accounts.ticket_metadata.key();
  sell_listing.ticket_owner = ctx.accounts.ticket_owner.key();
  Ok(())
}
