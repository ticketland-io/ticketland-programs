use anchor_lang::prelude::*;
use std::mem::size_of;
use anchor_spl::{
  token::{Mint, Token, TokenAccount},
  associated_token::AssociatedToken,
};
use anchor_metaplex::{
  mpl_token_metadata::{
    ID as metadata_id,
    state::{PREFIX},
  }
};
use common::{
  state::ticket_type::TicketType,
};
use crate::{
  utils::{
    program_error::ErrorCode,
  },
  account_data::{
    state::*,
    event::{*, SPACE_MARGIN},
  },
};

#[derive(Accounts)]
pub struct CreateEvent<'info> {
  #[account(mut)]
  pub state: Box<Account<'info, State>>,

  // The newly created event 
  #[account(
    init,
    payer = event_organizer,
    space = 8 + size_of::<Event>() + (size_of::<TicketType>() * MAX_TICKET_TYPES) + SPACE_MARGIN,
    seeds = [b"event", state.key().as_ref(), &state.n_events.to_string().as_ref()],
    bump
  )]
  pub event: Box<Account<'info, Event>>,

  /// CHECK: The authority of the event nfts
  #[account(
    mut,
    seeds = [b"event_nft_authority", state.key().as_ref()],
    bump = state.bumps.event_nft_authority,
  )]
  pub event_nft_authority: AccountInfo<'info>,

  #[account(
    init,
    payer = event_organizer,
    mint::decimals = 0,
    mint::authority = event_nft_authority,
    seeds = [b"event_nft", state.key().as_ref(), &state.n_events.to_string().as_ref()],
    bump,
  )]
  pub event_nft: Box<Account<'info, Mint>>,
  
  /// CHECK: The ATA that will receive the edition token which is needed for the master edition to be created
  /// This will be under the event organizer's control. However, the metadata will be controlled by the event_nft_authority
  #[account(
    init,
    payer = event_organizer,
    associated_token::mint = event_nft,
    associated_token::authority = event_organizer,
  )]
  pub organizer_event_nft_ata: Account<'info, TokenAccount>,

  /// CHECK: The metadata account that will be initialized in the processor
  #[account(
    mut,
    seeds = [PREFIX.as_bytes(), metadata_id.as_ref(), event_nft.key().as_ref()],
    seeds::program = metadata_id,
    bump,
  )]
  pub metadata: AccountInfo<'info>,

  /// CHECK: The edition account used when creating the the master edition account
  #[account(
    mut,
    seeds = [PREFIX.as_bytes(), metadata_id.as_ref(), event_nft.key().as_ref(), "edition".as_bytes()],
    seeds::program = metadata_id,
    bump,
  )]
  pub master_edition: AccountInfo<'info>,

  /// CHECK: The deposit token should be one of the supported currencies
  #[account(
    constraint = state.supported_currencies.iter().any(|c| c.mint_account == deposit_token.key()) @ ErrorCode::UnsupportedDepositToken
  )]
  pub deposit_token: Box<Account<'info, Mint>>,

  /// The deposit token ATA from which the event creation deposit will be transferred from
  #[account(
    mut,
    associated_token::mint = deposit_token,
    associated_token::authority = event_organizer,
  )]
  pub event_organizer_ata: Box<Account<'info, TokenAccount>>,

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

  #[account(mut)]
  pub event_organizer: Signer<'info>,
  
  pub token_program: Program<'info, Token>,
  associated_token_program: Program<'info, AssociatedToken>,
  // /// CHECK: The metadata program
  pub metadata_program: AccountInfo<'info>,
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
}
