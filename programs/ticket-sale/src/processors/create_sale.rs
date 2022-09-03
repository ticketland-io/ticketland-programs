use anchor_lang::prelude::*;
use common::{
	state::{
    ticket_type::TicketType,
  },
};
use crate::{
  context::create_sale::CreateSale,
  acl::{
    event_organizer,
  }, 
};

#[access_control(
  event_organizer::check(
    &ctx.accounts.event,
    &ctx.accounts.event_organizer,
  )
)]
pub fn exec(
  ctx: Context<CreateSale>,
  ticket_type_index: u8,
  event_id: [u8; 32],
  ticket_type: TicketType,
) -> Result<()> {
  let sale = &mut ctx.accounts.sale;

  sale.bump = *ctx.bumps.get("sale").unwrap();
  sale.event_id = event_id;
  sale.ticket_type_index = ticket_type_index;
  sale.ticket_type = ticket_type;

  Ok(())
}
