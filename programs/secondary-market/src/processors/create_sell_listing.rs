use anchor_lang::prelude::*;
use anchor_safe_math::SafeMath;
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
  require!(ticket_metadata.owner == ctx.accounts.ticket_owner.key(), ErrorCode::WrongTicketOwner);

  Ok(())
}

fn purchase_token_account_checks(ctx: &Context<CreateSellListing>) -> Result<()> {
  let event: Event = deser(ctx.accounts.event.clone())?;

  // The provided purchase token account should be the same as the one store in the event
  require!(ctx.accounts.purchase_token.key() == event.currency.mint_account, ErrorCode::WrongPurchaseToken);

  Ok(())
}

pub fn exec(
  ctx: Context<CreateSellListing>,
  market_id: [u8; 32],
  event_id: [u8; 32],
  ask_price: u64,
) -> Result<()> {
  ticket_metadata_account_checks(&ctx, event_id)?;
  purchase_token_account_checks(&ctx)?;

  let max_price = ask_price
    .safe_mul(ctx.accounts.market.resale_cap.safe_add(10_000)? as u64)?
    .safe_div(10_000)?;

  require!(ask_price <= max_price, ErrorCode::PriceCap);

  let sell_listing = &mut ctx.accounts.sell_listing;
  sell_listing.market_id = market_id;
  sell_listing.ask_price = ask_price;
  sell_listing.ticket_metadata = ctx.accounts.ticket_metadata.key();
  sell_listing.ticket_owner = ctx.accounts.ticket_owner.key();
  sell_listing.ticket_owner_purchase_token_ata = ctx.accounts.ticket_owner_purchase_token_ata.key();
  Ok(())
}
