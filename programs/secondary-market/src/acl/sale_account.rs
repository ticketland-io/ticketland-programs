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
  sale: &AccountInfo<'info>
) -> Result<()> {
  let ticket_metadata: TicketMetadata = deser(ticket_metadata.clone())?;
  require!(ticket_metadata.sale == sale.key(), ErrorCode::WrongSaleAccount);
  
  Ok(())
}
