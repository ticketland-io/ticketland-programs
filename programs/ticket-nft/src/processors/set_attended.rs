use anchor_lang::prelude::*;
use crate::{
  context::set_attended::*,
};

pub fn exec(ctx: Context<SetAttended>) -> Result<()> {
  let ticket_metadata = &mut ctx.accounts.ticket_metadata;

  ticket_metadata.attended = true;

  Ok(())
}
