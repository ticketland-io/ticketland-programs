use anchor_lang::prelude::*;
use common::{
  state::ticket_type::TicketType,
};
use crate::{
  program::EventRegistry,
  context::create_ticket_sale::CreateTicketSale, 
};

pub fn exec(
  ctx: Context<CreateTicketSale>,
  ticket_type_index: u8,
  ticket_type: TicketType,
) -> Result<()> {
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
    ticket_type_index,
		state.bumps.cpi_authority,
		event.id,
		ticket_type,
		EventRegistry::id(),
  )?;

  Ok(())
}
