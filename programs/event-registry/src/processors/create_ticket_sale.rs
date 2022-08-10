use anchor_lang::prelude::*;
use crate::{
  utils::program_error::ErrorCode,
  context::create_ticket_sale::CreateTicketSale, 
};

pub fn exec(
  ctx: Context<CreateTicketSale>,
  ticket_type_index: u8,
) -> Result<()> {
  require!(ticket_type_index <= ctx.accounts.event.ticket_types.len() as u8,  ErrorCode::WrongTicketTypeForIndex);

  let cpi_program = ctx.accounts.ticket_sale_program.to_account_info();
  let cpi_accounts = ticket_sale::cpi::accounts::CreateSale {
    state: ctx.accounts.ticket_sale_program_state.to_account_info(),
    sale: ctx.accounts.ticket_sale_state.to_account_info(),
    event_registry_cpi_authority: ctx.accounts.cpi_authority.to_account_info(),
    event_organizer: ctx.accounts.event_organizer.to_account_info(),
    system_program: ctx.accounts.system_program.to_account_info(),
    rent: ctx.accounts.rent.to_account_info(),
  };

  let state = &ctx.accounts.state;
  let state_key = state.key();

  let seeds: &[&[u8]] = &[
    b"cpi_authority", state_key.as_ref(),
    &[state.bumps.cpi_authority]
  ];
  let signer_seeds:&[&[&[u8]]] = &[&seeds[..]];

  let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
  let event = &ctx.accounts.event;
  let ticket_type = ctx.accounts.event.ticket_types[ticket_type_index as usize];
  
  ticket_sale::cpi::create_sale(
    cpi_ctx,
		state.bumps.cpi_authority,
    ticket_type_index,
		event.id,
		ticket_type,
  )?;

  Ok(())
}
