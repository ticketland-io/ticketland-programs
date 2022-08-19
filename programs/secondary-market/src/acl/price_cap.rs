use anchor_lang::{
  prelude::*,
};
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
  utils::program_error::ErrorCode,
};

pub fn check<'info>(
  ticket_metadata: &AccountInfo<'info>,
  resale_cap: u16,
  price: u64,
) -> Result<()> {
  let ticket_metadata: TicketMetadata = deser(ticket_metadata.clone())?;

  let max_price = ticket_metadata.price_sold
  .safe_mul(resale_cap.safe_add(10_000)? as u64)?
  .safe_div(10_000)?;

  require!(price <= max_price, ErrorCode::PriceCap);

  Ok(())
}
