use anchor_lang::prelude::*;
use crate::{
  context::transfer::*,
};

pub fn exec(ctx: Context<Transfer>, new_owner: Pubkey) -> Result<()> {
  let ticket_metadata = &mut ctx.accounts.ticket_metadata;

  ticket_metadata.owner = new_owner;

  Ok(())
}
