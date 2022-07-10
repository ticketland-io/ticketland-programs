use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
  program::invoke,
  system_instruction::transfer,
};
use anchor_spl::token::{self, Transfer, MintTo};
use anchor_safe_math::SafeMath;
use anchor_metaplex::{
  CreateMetadata,
  CreateMasterEdition,
  create_metadata,
  create_master_edition,
};
use common::{
  token::is_wrapped_sol,
  account_data::event::*,
  state::{
    alias::*,
    ticket_type::TicketType,
  },
};
use crate::{
  utils::program_error::ErrorCode,
  context::create_event::CreateEvent,
};

fn mint_edition_token(ctx: &Context<CreateEvent>, signer_seeds: &[&[&[u8]]]) -> Result<()> {
  let cpi_accounts = MintTo {
    mint: ctx.accounts.event_nft.to_account_info(),
    to: ctx.accounts.organizer_event_nft_ata.to_account_info(),
    authority: ctx.accounts.event_nft_authority.to_account_info(),
  };
  
  let cpi_program = ctx.accounts.token_program.to_account_info();
  let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
  
  token::mint_to(cpi_ctx, 1)
}

fn create_event_nft(
  ctx: &Context<CreateEvent>,
  signer_seeds: &[&[&[u8]]],
  name: String,
  symbol: String,
  uri: String,
  seller_fee_basis_points: u16,
) -> Result<()> {
  let cpi_accounts = CreateMetadata {
    mint: *ctx.accounts.event_nft.clone(),
    mint_authority: ctx.accounts.event_nft_authority.clone(),
    metadata_account: ctx.accounts.metadata.clone(),
    payer: ctx.accounts.event_organizer.to_account_info(),
    update_authority: ctx.accounts.event_nft_authority.to_account_info(),
    system_program: ctx.accounts.system_program.to_account_info(),
    rent: ctx.accounts.rent.to_account_info(),
  };

  create_metadata(
    cpi_accounts,
    signer_seeds,
    name.clone(),
    symbol.clone(),
    uri,
    None,
    seller_fee_basis_points,
    true,
    true,
    None,
    None
  )?;

  let cpi_accounts = CreateMasterEdition {
    edition: ctx.accounts.master_edition.clone(),
    mint: *ctx.accounts.event_nft.clone(),
    mint_authority: ctx.accounts.event_nft_authority.clone(),
    metadata_account: ctx.accounts.metadata.clone(),
    payer: ctx.accounts.event_organizer.to_account_info(),
    update_authority: ctx.accounts.event_nft_authority.to_account_info(),
    system_program: ctx.accounts.system_program.to_account_info(),
    rent: ctx.accounts.rent.to_account_info(),
  };

  create_master_edition(
    cpi_accounts,
    signer_seeds,
    Some(0),
  )
}

// Transfers Sol to the fund manager
fn lock_sol_deposit(ctx: &Context<CreateEvent>) -> Result<()> {
  let currency = &ctx.accounts.state.supported_currencies
    .iter()
    .find(|c| is_wrapped_sol(c.mint_account))
    .unwrap();

  let ix = transfer(
    &ctx.accounts.event_organizer.key(),
    &ctx.accounts.fund_manager.key(),
    currency.deposit_amount,
  );

  invoke(
    &ix,
    &[
      ctx.accounts.event_organizer.to_account_info(),
      ctx.accounts.fund_manager.to_account_info()
    ],
  ).map_err(|err| err.into())
}

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
  n_tickets: u32,
  start_time: Slot,
  end_time: Slot,
  purchase_token: Pubkey,
  ticket_types: Vec<TicketType>,
  name: String,
  symbol: String,
  uri: String,
) -> Result<()> {
  require!(ticket_types.len() <= MAX_TICKET_TYPES, ErrorCode::TooManyTicketTypes);
  require!(
    &ctx.accounts.state.supported_currencies
      .iter()
      .find(|c| c.mint_account == purchase_token)
      .is_some(),
    ErrorCode::UnsupportedPurchaseToken,
  );

  if is_wrapped_sol(ctx.accounts.deposit_token.key()) {
    lock_sol_deposit(&ctx)?;
  } else {
    lock_deposit(&ctx)?;
  }

  {
    let event = &mut ctx.accounts.event;
    let state = &mut ctx.accounts.state;

    event.bumps = EventBumps {
      event: *ctx.bumps.get("event").unwrap(),
      event_nft: *ctx.bumps.get("event_nft").unwrap(),
    };

    event.id = state.n_events;
    event.n_tickets = n_tickets;
    event.start_time = start_time;
    event.end_time = end_time;
    event.purchase_token = purchase_token;
    event.event_organizer = ctx.accounts.event_organizer.key();
    event.ticket_types = ticket_types;
    event.event_capacity = ctx.accounts.event_capacity.key();

    state.n_events = state.n_events.safe_add(1)?;
  }

  let state = &ctx.accounts.state;
  let state_key = state.key();

  let seeds: &[&[u8]] = &[
    b"event_nft_authority", state_key.as_ref(),
    &[state.bumps.event_nft_authority]
  ];
  let signer_seeds:&[&[&[u8]]] = &[&seeds[..]];

  // We need this otherwise we get this error "Editions must have exactly one token"
  mint_edition_token(&ctx, signer_seeds)?;
  create_event_nft(&ctx, signer_seeds, name, symbol, uri, state.seller_fee_basis_points)?;
  init_event_capacity(&ctx)?;

  Ok(())
}
