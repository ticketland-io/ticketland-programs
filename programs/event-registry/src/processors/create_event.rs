use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo};
use anchor_safe_math::SafeMath;
use anchor_metaplex::{
  CreateMetadata,
  CreateMasterEdition,
  create_metadata,
  create_master_edition,
};
use common::{
  state::{
    alias::*,
    ticket_type::TicketType,
  },
};
use crate::{
  utils::program_error::ErrorCode,
  context::create_event::CreateEvent, 
  account_data::event::*,
};

fn mint_edition_token(ctx: &Context<CreateEvent>, signer_seeds: &[&[&[u8]]]) -> Result<()> {
  let cpi_accounts = MintTo {
    mint: ctx.accounts.event_nft.to_account_info(),
    to: ctx.accounts.event_nft_authority_ata.to_account_info(),
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
    mint: ctx.accounts.event_nft.clone(),
    mint_authority: ctx.accounts.event_nft_authority.clone(),
    metadata_account: ctx.accounts.metadata.clone(),
    payer: ctx.accounts.event_organizer.to_account_info(),
    update_authority: ctx.accounts.event_nft_authority_ata.to_account_info(),
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
    mint: ctx.accounts.event_nft.clone(),
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

fn check_deposit(ctx: &Context<CreateEvent>) -> Result<()> {
  let fund_manager_ata = &ctx.accounts.fund_manager_ata;
  let deposit_token = &ctx.accounts.deposit_token;
  let currency = &ctx.accounts.state.supported_currencies
    .iter()
    .find(|c| c.mint_account == deposit_token.key())
    .unwrap();

  // Check that enough tokens are deposited to the fund manager ata
  if fund_manager_ata.amount < (**currency).deposit_amount {
    return Err(ErrorCode::NotEnoughDeposit.into())
  }

  Ok(())
}

pub fn exec(
  ctx: Context<CreateEvent>,
  start_time: Slot,
  end_time: Slot,
  sale_start_time: Vec<Slot>,
  ticket_types: Vec<TicketType>,
  name: String,
  symbol: String,
  uri: String,
) -> Result<()> {
  require!(ticket_types.len() <= MAX_TICKET_TYPES, ErrorCode::TooManyTicketTypes);

  check_deposit(&ctx)?;

  {
    let event = &mut ctx.accounts.event;
    let state = &mut ctx.accounts.state;

    event.id = state.n_events;
    event.start_time = start_time;
    event.end_time = end_time;
    event.ticket_types = ticket_types.clone();
    event.event_organizer = ctx.accounts.event_organizer.key();

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

  // Create a new sale for each ticket type
  // for ticket_type in ticket_types {
  //   let cpi_program = ctx.accounts.ticket_sale_program.to_account_info();
  // }

  Ok(())
}
