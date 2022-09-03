use anchor_lang::prelude::*;
use common::{
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

fn event_account_checks(ctx: &Context<CreateSale>, event: &Event, ticket_type_index: u8,) -> Result<()> {
  // The provided event organizer account should be the same as the one stored in the event
  require!(ctx.accounts.event_organizer.key() == event.event_organizer, ErrorCode::WrongEventOrganizer);
  require!(ticket_type_index < event.ticket_types.len() as u8, ErrorCode::InvalidTicketTypeIndex);

  Ok(())
}

pub fn exec(
  ctx: Context<CreateSale>,
  ticket_type_index: u8,
  event_id: [u8; 32],
) -> Result<()> {
  let event: Event = deser(ctx.accounts.event.clone())?;
  event_account_checks(&ctx, &event, ticket_type_index)?;

  let ticket_type = event.ticket_types[ticket_type_index as usize];
  let sale = &mut ctx.accounts.sale;

  sale.bump = *ctx.bumps.get("sale").unwrap();
  sale.event_id = event_id;
  sale.ticket_type_index = ticket_type_index;
  sale.ticket_type = ticket_type;

  Ok(())
}
