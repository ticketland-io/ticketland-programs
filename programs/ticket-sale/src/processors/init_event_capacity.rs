use anchor_lang::prelude::*;
use crate::{
  context::init_event_capacity::InitEventCapacity,
};

pub fn exec(
  ctx: Context<InitEventCapacity>,
  event_id: [u8; 32],
  n_tickets: u32,
) -> Result<()> {
  let event_capacity = &mut ctx.accounts.event_capacity.load_init()?;

  if !event_capacity.is_initialized {
    event_capacity.event_id = event_id;
    event_capacity.is_initialized = true;
    event_capacity.available_tickets = n_tickets;
  }

  Ok(())
}
