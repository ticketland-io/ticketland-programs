use anchor_lang::prelude::*;
use common::{
	state::ticket_type::TicketType,
};
use crate::{
  context::create_sale::CreateSale,
};

pub fn exec(
  ctx: Context<CreateSale>,
  ticket_type_index: u8,
  ticket_type: TicketType,
) -> Result<()> {
  let sale = &mut ctx.accounts.sale;

  sale.event_id = ctx.accounts.event_registry_state.n_events;
  sale.ticket_type_index = ticket_type_index;
  sale.ticket_type = ticket_type;

  Ok(())
}
