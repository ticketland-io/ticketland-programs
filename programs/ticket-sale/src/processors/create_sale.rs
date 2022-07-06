use anchor_lang::prelude::*;
use common::{
	state::{
    alias::*,
    ticket_type::TicketType,
  },
};
use crate::{
  context::create_sale::CreateSale,
};

pub fn exec(
  ctx: Context<CreateSale>,
  ticket_type_index: usize,
  event_id: u64,
  ticket_type: TicketType,
  sale_start_time: Slot,
) -> Result<()> {
  let sale = &mut ctx.accounts.sale;

  sale.event_id = event_id;
  sale.ticket_type_index = ticket_type_index;
  sale.ticket_type = ticket_type;
  sale.sale_start_time = sale_start_time;

  Ok(())
}
