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
  context::cancel_sell_listing::*,
  utils::program_error::ErrorCode,
};

/// Check that the as ticket metadata account has the same event id as the one that was passed as param.
/// Additionally, check that the owner of the ticket metadata (and thus the owner of the ticket nft) is the signer
/// of this tx
fn ticket_metadata_account_checks<'info>(
  ticket_metadata: &AccountInfo<'info>,
  ticket_owner: &Signer<'info>,
  event_id: [u8; 32],
) -> Result<()> {
  let ticket_metadata: TicketMetadata = deser(ticket_metadata.clone())?;

  require!(ticket_metadata.event_id == event_id, ErrorCode::EventIdDoesNotMatch);
  require!(ticket_metadata.owner == ticket_owner.key(), ErrorCode::OnlyTicketOwner);

  Ok(())
}

#[access_control(
  ticket_metadata_account_checks(
    &ctx.accounts.ticket_metadata,
    &ctx.accounts.ticket_owner,
    event_id
  )
)]
pub fn exec(ctx: Context<CancelSellListing>, event_id: [u8; 32]) -> Result<()> {
  Ok(())
}
