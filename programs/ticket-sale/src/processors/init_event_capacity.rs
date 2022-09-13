use anchor_lang::prelude::*;
use common::{
  utils::bitmap,
  account_data::{
    serialization::deser,
  },
};
use crate::{
  ID,
  account_data::event_capacity::EventCapacity,
  context::init_event_capacity::InitEventCapacity,
  utils::program_error::ErrorCode,
};

pub fn exec(
  ctx: Context<InitEventCapacity>,
  event_id: [u8; 32],
  n_tickets: u32,
) -> Result<()> {
  require!(ctx.accounts.event_capacity.owner == &ID, ErrorCode::NotOwnedByThisProgram);
  let mut event_capacity: EventCapacity = deser(ctx.accounts.event_capacity.clone())?;

  if !event_capacity.is_initialized {
    event_capacity.event_id = event_id;
    event_capacity.is_initialized = true;
    event_capacity.available_tickets = n_tickets;
    event_capacity.seats = vec![0; bitmap::count_to_len(n_tickets)];
  }

  Ok(())
}
