use anchor_lang::{
  prelude::*,
};
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
  utils::program_error::ErrorCode,
};

pub fn check<'info>(
  ticket_metadata: &AccountInfo<'info>,
  ticket_owner: &AccountInfo<'info>
) -> Result<()> {
  let ticket_metadata: TicketMetadata = deser(ticket_metadata.clone())?;
  require!(ticket_metadata.owner == ticket_owner.key(), ErrorCode::OnlyTicketOwner);
  
  Ok(())
}
