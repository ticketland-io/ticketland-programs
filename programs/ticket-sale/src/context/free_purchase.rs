use anchor_lang::prelude::*;
use anchor_spl::{
  token::{Mint, Token},
  associated_token::AssociatedToken,
};
use ticket_nft::{
  program::TicketNft,
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
    seat_verification::*,
  },
  utils::program_error::ErrorCode,
};

#[derive(Accounts)]
#[instruction(seat_index: u32, seat_name: String)]
pub struct FreePurchase<'info> {
  #[account(mut)]
  pub state: Box<Account<'info, State>>,

  /// CHECK: The Event account
  #[account(
    seeds = [
      b"event",
      state.event_registry_state.key().as_ref(),
      &sale.event_id
    ],
    bump,
    seeds::program = state.event_registry_program,
  )]
  pub event: AccountInfo<'info>,

  #[account(
    mut,
    close = ticket_buyer,
    seeds = [
      b"seat_verification",
      state.key().as_ref(),
      seat_index.to_string().as_ref(),
      seat_name.as_ref(),
    ],
    bump = seat_verification.bump,
    constraint = seat_verification.verified @ ErrorCode::SeatNotVerified,
  )]
  pub seat_verification: Account<'info, SeatVerification>,

  #[account(
    mut,
    seeds = [
      b"sale",
      state.key().as_ref(),
      sale.ticket_type_index.to_string().as_ref(),
      &sale.event_id
    ],
    bump = sale.bump,
    constraint = sale.ticket_type.sale_type.is_free() @ ErrorCode::UnexpectedSaleAccount,
  )]
  pub sale: Box<Account<'info, Sale>>,

  /// CHECK: THe PDA that will be sending CPI to other programs i.e. Ticket NFT Program
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
  )]
  pub event_capacity: Account<'info, EventCapacity>,

  /// CHECK: This is the event organizer of the event
  #[account()]
  pub event_organizer: AccountInfo<'info>,

  #[account(mut)]
  pub ticket_buyer: Signer<'info>,

  // ------ These account are needed for the CPI to the Ticket NFT program ------

  /// CHECK: The state of the ticker program sale
  #[account(
    mut,
    constraint = ticket_nft_program_state.owner.key() == ticket_nft_program.key() @ ErrorCode::WrongTicketNftProgramStateAccount
  )]
  pub ticket_nft_program_state: AccountInfo<'info>,

  /// CHECK: The underlying Ticket NFT Mint account
  /// We use AccountInfo instead of Account<'info, Mint> because the latter will check if the 
  /// account is initialized. However, this account will be initialized in the Ticket NFT program after
  /// the CPI is done.
  #[account(
    mut,
    seeds = [
      b"ticket_nft",
      ticket_nft_program_state.key().as_ref(),
      seat_index.to_string().as_ref(),
      &sale.event_id
    ],
    bump,
    seeds::program = ticket_nft_program.key(),
  )]
  pub ticket_nft: AccountInfo<'info>,

  /// CHECK: The authority of all NFTs
  #[account(
    seeds = [b"nft_authority", ticket_nft_program_state.key().as_ref()],
    bump,
    seeds::program = ticket_nft_program.key(),
  )]
  pub nft_authority: AccountInfo<'info>,

  /// CHECK: The newly created Ticket Metadata 
  #[account(
    mut,
    seeds = [
      b"ticket_metadata",
      ticket_nft_program_state.key().as_ref(),
      ticket_nft.key().as_ref(),
    ],
    bump,
    seeds::program = ticket_nft_program.key(),
  )]
  pub ticket_metadata: AccountInfo<'info>,

  /// CHECK: The NFT master edition account
  #[account(
    mut,
    seeds = [PREFIX.as_bytes(), metadata_id.as_ref(), ticket_nft.key().as_ref(), "edition".as_bytes()],
    seeds::program = metadata_id,
    bump,
  )]
  pub master_edition: AccountInfo<'info>,

  /// CHECK: The metaplex metadata account that will be initialized in the processor
  #[account(
    mut,
    seeds = [PREFIX.as_bytes(), metadata_id.as_ref(), ticket_nft.key().as_ref()],
    seeds::program = metadata_id,
    bump,
  )]
  pub ticket_metaplex_metadata: AccountInfo<'info>,

  /// CHECK:
  /// The ATA that is a PDA controlled by this program and will be the owner of the Ticket NFT
  /// until the end of the event.
  /// We use AccountInfo for the reason explained above. Also we can add these constraints.
  /// 
  /// associated_token::mint = ticket_nft,
  /// associated_token::authority = cpi_authority,
  /// 
  /// If we add them then Anchor would have to load the TokenAccount and check these constraints. However, there is
  /// not account yet; it will be created in the Ticket NFT Program
  #[account(mut)]
  pub ticket_nft_ata: AccountInfo<'info>,

  #[account(
    seeds = [b"event_nft", state.event_registry_state.key().as_ref(), &sale.event_id],
    bump,
    seeds::program = state.event_registry_program,
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

  /// CHECK: This is the ticket sale program account
  pub ticket_nft_program: Program<'info, TicketNft>,

  pub token_program: Program<'info, Token>,
  pub associated_token_program: Program<'info, AssociatedToken>,
  /// CHECK: The metadata program
  pub metadata_program: AccountInfo<'info>,
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
}
