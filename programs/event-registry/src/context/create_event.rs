use anchor_lang::prelude::*;
use std::mem::size_of;
use anchor_spl::{
  token::{Mint, Token, TokenAccount},
  associated_token::AssociatedToken,
};
use common::{
  account_data::event::{
    MAX_TICKET_TYPES,
  },
  state::ticket_type::{TicketType, MAX_NAME_LENGTH},
};
use ticket_sale::{
  program::TicketSale,
};
use crate::{
  utils::{
    program_error::ErrorCode,
  },
  account_data::{
    state::*,
    event::*,
  },
};

#[derive(Accounts)]
#[instruction(event_id: [u8; 32])]
pub struct CreateEvent<'info> {
  #[account(mut)]
  pub state: Box<Account<'info, State>>,

  // The newly created event
  #[account(
    init,
    payer = event_organizer,
    space = 8 + size_of::<Event>() + ((size_of::<TicketType>() + MAX_NAME_LENGTH) * MAX_TICKET_TYPES),
    seeds = [b"event", state.key().as_ref(), &event_id],
    bump
  )]
  pub event: Box<Account<'info, Event>>,

  /// CHECK: The deposit token should be one of the supported currencies
  #[account(
    constraint = state.supported_currencies.iter().any(|c| c.mint_account == deposit_token.key()) @ ErrorCode::UnsupportedDepositToken
  )]
  pub deposit_token: Box<Account<'info, Mint>>,

  /// CHECK: The deposit token should be one of the supported currencies
  #[account(
    constraint = state.supported_currencies.iter().any(|c| c.mint_account == purchase_token.key()) @ ErrorCode::UnsupportedPurchaseToken
  )]
  pub purchase_token: Box<Account<'info, Mint>>,

  /// The deposit token ATA from which the event creation deposit will be transferred from
  #[account(
    mut,
    associated_token::mint = deposit_token,
    associated_token::authority = event_organizer,
  )]
  pub event_organizer_ata: Box<Account<'info, TokenAccount>>,

  /// The ATA where the funds from selling the tickets will be transferred to
  #[account(
    mut,
    associated_token::mint = purchase_token,
    associated_token::authority = event_organizer,
  )]
  pub event_organizer_purchase_token_ata: Box<Account<'info, TokenAccount>>,

  /// CHECK: The PDA that will be the authority to handle all deposits for the given event_organizer
  /// Each user will have his own PDA
  #[account(
    init_if_needed,
    payer = event_organizer,
    space = 0,
    seeds = [b"fund_manager", state.key().as_ref(), event.key().as_ref(), event_organizer.key().as_ref()],
    bump,
  )]
  pub fund_manager: AccountInfo<'info>,

  /// The ATA that holds creators deposit in the given deposit token
  #[account(
    init_if_needed,
    payer = event_organizer,
    associated_token::mint = deposit_token,
    associated_token::authority = fund_manager,
  )]
  pub fund_manager_ata: Box<Account<'info, TokenAccount>>,

  /// CHECK: The account that will hold the seats bitmap. It cannot be PDA due to space limitations.
  #[account(
    mut,
    constraint = *event_capacity.owner == ticket_sale_program.key() @ ErrorCode::TicketSaleMustBeOwner,
  )]
  pub event_capacity: AccountInfo<'info>,

  /// CHECK: The state of the ticker program sale
  #[account(
    constraint = ticket_sale_program_state.owner.key() == ticket_sale_program.key() @ ErrorCode::WrongTicketSaleProgramStateAccount
  )]
  pub ticket_sale_program_state: AccountInfo<'info>,

  /// CHECK: THe PDA that will be sending CPI to other programs i.e. TicketSale Program
  #[account(
    mut,
    seeds = [b"cpi_authority", state.key().as_ref()],
    bump = state.bumps.cpi_authority,
  )]
  pub cpi_authority: AccountInfo<'info>,

  #[account(mut)]
  pub event_organizer: Signer<'info>,
  
  pub ticket_sale_program: Program<'info, TicketSale>,
  pub token_program: Program<'info, Token>,
  associated_token_program: Program<'info, AssociatedToken>,
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
}
