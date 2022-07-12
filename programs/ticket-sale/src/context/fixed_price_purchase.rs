use anchor_lang::prelude::*;
use anchor_spl::{
  token::{Mint, Token, TokenAccount},
  associated_token::AssociatedToken,
};
use ticket_nft::{
  program::TicketNft,
  account_data::ticket_metadata::TicketMetadata,
};
use anchor_metaplex::{
  mpl_token_metadata::{
    ID as metadata_id,
    state::{PREFIX},
  }
};
use crate::{
  ID,
  account_data::{
    state::*,
    event_capacity::*,
    sale::*,
    event::*,
  },
  utils::program_error::ErrorCode,
};

#[derive(Accounts)]
pub struct FixedPricePurchase<'info> {
  #[account(mut)]
  pub state: Account<'info, State>,

  // The newly created event
  #[account(
    seeds = [
      b"event",
      state.event_registry_state.key().as_ref(),
      &event_capacity.load()?.event_id.to_string().as_ref()
    ],
    bump,
    constraint = event.id == sale.event_id @ ErrorCode::WrongEventAccount,
  )]
  pub event: Box<Account<'info, Event>>,

  #[account(
    mut,
    seeds = [
      b"sale",
      state.key().as_ref(),
      sale.ticket_type_index.to_string().as_ref(),
      sale.event_id.to_string().as_ref()
    ],
    bump = sale.bump,
    constraint = !sale.ticket_type.sale_type.is_fixed_price() @ ErrorCode::ExpectedFixedPriceSaleAccount,
  )]
  pub sale: Account<'info, Sale>,

  /// CHECK: THe PDA that will be sending CPI to other programs i.e. TicketSale Program
  #[account(
    mut,
    seeds = [b"ticket_sale:cpi_authority", state.key().as_ref()],
    bump = state.bumps.cpi_authority,
  )]
  pub cpi_authority: AccountInfo<'info>,

  /// CHECK: The account that will hold the seats bitmap
  #[account(
    mut,
    constraint = EventCapacity::owner() == ID @ ErrorCode::NotOwnedByThisProgram,
    constraint = event_capacity.key() == sale.event_capacity @ ErrorCode::WrongEventCapacityAccount,
  )]
  pub event_capacity: AccountLoader<'info, EventCapacity>,

  /// CHECK: The deposit token should be one of the supported currencies
  #[account(
    constraint = event.currency.mint_account == purchase_token.key() @ ErrorCode::UnsupportedPurchaseToken
  )]
  pub purchase_token: Box<Account<'info, Mint>>,

  /// The deposit token ATA from which the event creation deposit will be transferred from
  #[account(
    mut,
    associated_token::mint = purchase_token,
    associated_token::authority = event_organizer,
  )]
  pub event_organizer_purchase_token_ata: Box<Account<'info, TokenAccount>>,

  /// CHECK: The Sol account that organizer will receive funds from the ticket sale  to
  #[account(
    mut,
    constraint = event_organizer_purchase_sol_treasury.key() == event.event_organizer_treasury @ ErrorCode::WrongSolTreasury,
  )]
  pub event_organizer_purchase_sol_treasury: AccountInfo<'info>,

  /// CHECK: This is the event organizer of the event
  #[account(
    constraint = event_organizer.key() == event.event_organizer @ ErrorCode::WrongEventOrganizer,
  )]
  pub event_organizer: AccountInfo<'info>,

  /// The deposit token ATA from which the event creation deposit will be transferred from
  #[account(
    mut,
    associated_token::mint = purchase_token,
    associated_token::authority = treasury,
  )]
  pub service_fee_ata: Box<Account<'info, TokenAccount>>,

  /// CHECK: This is ticketland.io treasury address
  #[account(
    constraint = treasury.key() == state.treasury @ ErrorCode::WrongTreasuryAccount,
  )]
  pub treasury: AccountInfo<'info>,

  /// The ticket buyer ATA from which funds will be sent
  #[account(
    mut,
    associated_token::mint = purchase_token,
    associated_token::authority = ticket_buyer,
  )]
  pub ticket_buyer_ata: Box<Account<'info, TokenAccount>>,

  #[account(mut)]
  pub ticket_buyer: Signer<'info>,

  // ------ These account are needed for the CPI to the Ticket NFT program ------

  /// CHECK: The state of the ticker program sale
  #[account(
    mut,
    constraint = ticket_nft_program_state.owner.key() == ticket_nft_program.key() @ ErrorCode::WrongTicketNftProgramStateAccount
  )]
  pub ticket_nft_program_state: AccountInfo<'info>,

  /// The underlying Ticket NFT Mint account
  #[account(
    seeds = [
      b"ticket_nft",
      ticket_nft_program_state.key().as_ref(),
      ticket_buyer.key().as_ref(),
      &event.id.to_string().as_ref()
    ],
    bump,
  )]
  pub ticket_nft: Box<Account<'info, Mint>>,

  /// CHECK: The authority of all NFTs
  #[account(
    seeds = [b"nft_authority", ticket_nft_program_state.key().as_ref()],
    bump,
  )]
  pub nft_authority: AccountInfo<'info>,

  /// The newly created Ticket Metadata 
  #[account(
    seeds = [
      b"ticket_metadata",
      ticket_nft_program_state.key().as_ref(),
      ticket_nft.key().as_ref(),
    ],
    bump
  )]
  pub ticket_metadata: Box<Account<'info, TicketMetadata>>,

  /// CHECK: The metaplex metadata account that will be initialized in the processor
  #[account(
    mut,
    seeds = [PREFIX.as_bytes(), metadata_id.as_ref(), ticket_nft.key().as_ref()],
    seeds::program = metadata_id,
    bump,
  )]
  pub ticket_metaplex_metadata: AccountInfo<'info>,

  /// The ATA that is a PDA controlled by the Ticket sale program and will be the owner of the Ticket NFT
  /// until the end of the event.
  #[account(
    associated_token::mint = ticket_nft,
    associated_token::authority = cpi_authority,
  )]
  pub ticket_nft_ata: Box<Account<'info, TokenAccount>>,

  #[account(
    seeds = [b"event_nft", state.event_registry_state.key().as_ref(), &event.id.to_string().as_ref()],
    bump,
  )]
  pub event_nft: Box<Account<'info, Mint>>,

  /// CHECK: The metadata account that will be initialized in the processor
  #[account(
    mut,
    seeds = [PREFIX.as_bytes(), metadata_id.as_ref(), event_nft.key().as_ref()],
    seeds::program = metadata_id,
    bump,
  )]
  pub event_nft_metadata: AccountInfo<'info>,

  associated_token_program: Program<'info, AssociatedToken>,
  /// CHECK: This is the ticket sale program account
  pub ticket_nft_program: Program<'info, TicketNft>,

  pub token_program: Program<'info, Token>,
}
