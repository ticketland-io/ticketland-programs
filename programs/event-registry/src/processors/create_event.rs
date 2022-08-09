use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};
use anchor_safe_math::SafeMath;
use common::{
  account_data::event::*,
  state::{
    ticket_type::TicketType,
  },
};
use crate::{
  utils::program_error::ErrorCode,
  context::create_event::CreateEvent,
};

/// Transfer the deposit amount to the fund manager ata
fn lock_deposit(ctx: &Context<CreateEvent>) -> Result<()> {
  let cpi_accounts = Transfer {
    from: ctx.accounts.event_organizer_ata.to_account_info(),
    to: ctx.accounts.fund_manager_ata.to_account_info(),
    authority: ctx.accounts.event_organizer.to_account_info(),
  };
  let cpi_program = ctx.accounts.token_program.to_account_info();
  let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    
  let currency = &ctx.accounts.state.supported_currencies
    .iter()
    .find(|c| c.mint_account == ctx.accounts.deposit_token.key())
    .unwrap();

  token::transfer(cpi_ctx, currency.deposit_amount)
}

fn init_event_capacity(ctx: &Context<CreateEvent>,) -> Result<()> {
  let state = &ctx.accounts.state;
  let state_key = state.key();

  let seeds: &[&[u8]] = &[
    b"cpi_authority", state_key.as_ref(),
    &[state.bumps.cpi_authority]
  ];
  let signer_seeds:&[&[&[u8]]] = &[&seeds[..]];

  let cpi_program = ctx.accounts.ticket_sale_program.to_account_info();
  let cpi_accounts = ticket_sale::cpi::accounts::InitEventCapacity {
    state: ctx.accounts.ticket_sale_program_state.to_account_info(),
    event_capacity: ctx.accounts.event_capacity.to_account_info(),
    event_registry_cpi_authority: ctx.accounts.cpi_authority.to_account_info(),
  };

  let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
  let event = &ctx.accounts.event;

  ticket_sale::cpi::init_event_capacity(
    cpi_ctx,
		state.bumps.cpi_authority,
		event.id,
		event.n_tickets,
  )?;
  
  Ok(())
}

pub fn exec(
  ctx: Context<CreateEvent>,
  event_id: [u8; 32],
  n_tickets: u32,
  start_time: i64,
  end_time: i64,
  ticket_types: Vec<TicketType>,
) -> Result<()> {
  require!(ticket_types.len() <= MAX_TICKET_TYPES, ErrorCode::TooManyTicketTypes);
  
  lock_deposit(&ctx)?;

  {
    let event = &mut ctx.accounts.event;
    let state = &mut ctx.accounts.state;

    event.bumps = EventBumps {
      event: *ctx.bumps.get("event").unwrap(),
      event_nft: *ctx.bumps.get("event_nft").unwrap(),
    };

    event.id = event_id;
    event.n_tickets = n_tickets;
    event.start_time = start_time;
    event.end_time = end_time;
    event.currency = *state.supported_currencies.iter().find(|c| c.mint_account == ctx.accounts.purchase_token.key()).unwrap();
    event.event_organizer_purchase_token_ata = ctx.accounts.event_organizer_purchase_token_ata.key();
    event.event_organizer = ctx.accounts.event_organizer.key();
    event.ticket_types = ticket_types;
    event.event_capacity = ctx.accounts.event_capacity.key();

    state.n_events = state.n_events.safe_add(1)?;
  }

  init_event_capacity(&ctx)?;

  Ok(())
}
