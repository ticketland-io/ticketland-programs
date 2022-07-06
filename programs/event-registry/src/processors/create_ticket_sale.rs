use anchor_lang::prelude::*;
use common::{
  state::{
    ticket_type::TicketType,
  },
};
use crate::{
  program::EventRegistry,
  utils::program_error::ErrorCode,
  context::create_ticket_sale::CreateTicketSale, 
};

pub fn exec(
  ctx: Context<CreateTicketSale>,
  ticket_type_index: usize,
  ticket_type: TicketType,
) -> Result<()> {
  {
    let ticket_type_at_index = &ctx.accounts.event.ticket_types[ticket_type_index as usize];
    require!(*ticket_type_at_index == ticket_type,  ErrorCode::WrongTicketTypeForIndex);
  }

  let cpi_program = ctx.accounts.ticket_sale_program.to_account_info();
  let cpi_accounts = ticket_sale::cpi::accounts::CreateSale {
    state: ctx.accounts.ticket_sale_program_state.to_account_info(),
    event_registry_state: ctx.accounts.state.to_account_info(),
    sale: ctx.accounts.ticket_sale_state.to_account_info(),
    event_registry_cpi_authority: ctx.accounts.cpi_authority.to_account_info(),
    event_organizer: ctx.accounts.event_organizer.to_account_info(),
    system_program: ctx.accounts.system_program.to_account_info(),
    rent: ctx.accounts.rent.to_account_info(),
  };

  let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
  let state = &ctx.accounts.state;
  let event = &ctx.accounts.event;

  ticket_sale::cpi::create_sale(
    cpi_ctx,
		state.bumps.cpi_authority,
		EventRegistry::id(),
    ticket_type_index,
		event.id,
		ticket_type,
  )?;

  Ok(())
}
