use anchor_lang::prelude::*;
use common::{
	state::{
    ticket_type::TicketType,
  },
};
use crate::{
  context::create_sale::CreateSale,
};

fn init_event_capacity(
  ctx: &Context<CreateSale>,
  event_id: u64,
  n_tickets: u32,
) -> Result<()> {
  let event_capacity = &mut ctx.accounts.event_capacity.load_mut()?;

  if !event_capacity.is_initialized {
    event_capacity.event_id = event_id;
    event_capacity.is_initialized = true;
    event_capacity.available_tickets = n_tickets;
  }

  Ok(())
}

pub fn exec(
  ctx: Context<CreateSale>,
  ticket_type_index: usize,
  event_id: u64,
  n_tickets: u32,
  ticket_type: TicketType,
) -> Result<()> {
  init_event_capacity(&ctx, event_id, n_tickets)?;

  let sale = &mut ctx.accounts.sale;

  sale.event_id = event_id;
  sale.ticket_type_index = ticket_type_index;
  sale.ticket_type = ticket_type;

  Ok(())
}
