use anchor_lang::prelude::*;
use common::{
	state::{
    ticket_type::TicketType,
  },
  account_data::{
    serialization::deser,
  },
};
use crate::{
  context::create_sale::CreateSale,
  account_data::{
    event::*,
  },
  utils::{
    program_error::ErrorCode,
  }
};

fn event_account_checks(ctx: &Context<CreateSale>, ticket_type_index: u8,) -> Result<()> {
  let event: Event = deser(ctx.accounts.event.clone())?;

  // The provided event organizer account should be the same as the one stored in the event
  require!(ctx.accounts.event_organizer.key() == event.event_organizer, ErrorCode::WrongEventOrganizer);
  require!(ticket_type_index < event.ticket_types.len() as u8, ErrorCode::InvalidTicketTypeIndex);

  Ok(())
}

pub fn exec(
  ctx: Context<CreateSale>,
  ticket_type_index: u8,
  event_id: [u8; 32],
  ticket_type: TicketType,
) -> Result<()> {
  event_account_checks(&ctx, ticket_type_index)?;

  let sale = &mut ctx.accounts.sale;

  sale.bump = *ctx.bumps.get("sale").unwrap();
  sale.event_id = event_id;
  sale.ticket_type_index = ticket_type_index;
  sale.ticket_type = ticket_type;

  Ok(())
}
