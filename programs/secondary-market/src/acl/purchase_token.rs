use anchor_lang::{
  prelude::*,
};
use anchor_spl::{
  token::{Mint},
};
use common::{
  account_data::{
    serialization::deser,
  },
};

use crate::{
  account_data::{
    event::*,
  },
  utils::program_error::ErrorCode,
};

pub fn check<'info>(
  event: &AccountInfo<'info>,
  purchase_token: &Account<'info, Mint>,
) -> Result<()> {
  let event: Event = deser(event.clone())?;

  // The provided purchase token account should be the same as the one store in the event
  require!(purchase_token.key() == event.currency.mint_account, ErrorCode::WrongPurchaseToken);

  Ok(())
}
