use anchor_lang::{
  prelude::*,
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
  event_organizer: &AccountInfo<'info>,
) -> Result<()> {
  let event: Event = deser(event.clone())?;

  // The provided event organizer account should be the same as the one stored in the event
  require!(event_organizer.key() == event.event_organizer, ErrorCode::OnlyEventOrganizer);

  Ok(())
}
