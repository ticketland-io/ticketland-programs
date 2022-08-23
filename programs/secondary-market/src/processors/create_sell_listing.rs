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
  account_data::{
    event::*,
  },
  acl::{
    sale_time_checks,
    sale_account,
    price_cap,
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

fn purchase_token_account_checks(ctx: &Context<CreateSellListing>) -> Result<()> {
  let event: Event = deser(ctx.accounts.event.clone())?;

  // The provided purchase token account should be the same as the one store in the event
  require!(ctx.accounts.purchase_token.key() == event.currency.mint_account, ErrorCode::WrongPurchaseToken);

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
)]
pub fn exec(
  ctx: Context<CreateSellListing>,
  event_id: [u8; 32],
  ask_price: u64,
) -> Result<()> {
  ticket_metadata_account_checks(&ctx, event_id)?;
  purchase_token_account_checks(&ctx)?;

  let sell_listing = &mut ctx.accounts.sell_listing;
  sell_listing.ask_price = ask_price;
  sell_listing.ticket_metadata = ctx.accounts.ticket_metadata.key();
  sell_listing.ticket_owner = ctx.accounts.ticket_owner.key();
  sell_listing.ticket_owner_purchase_token_ata = ctx.accounts.ticket_owner_purchase_token_ata.key();
  Ok(())
}
