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
use crate::{
  account_data::{
    state::*,
    ticket_metadata::{TicketMetadata, SPACE_MARGIN},
  },
};

#[derive(Accounts)]
#[instruction(cpi_authority_bump: u8, event_id: [u8; 32])]
pub struct CreateTicket<'info> {
  #[account()]
  pub state: Box<Account<'info, State>>,

  /// The newly created Ticket Metadata 
  #[account(
    init,
    payer = ticket_buyer,
    space = 8 + size_of::<TicketMetadata>() + SPACE_MARGIN,
    seeds = [
      b"ticket_metadata",
      state.key().as_ref(),
      nft.key().as_ref(),
    ],
    bump
  )]
  pub ticket_metadata: Box<Account<'info, TicketMetadata>>,

  /// CHECK: The authority of all NFTs
  #[account(
    seeds = [b"nft_authority", state.key().as_ref()],
    bump = state.bumps.nft_authority,
  )]
  pub nft_authority: AccountInfo<'info>,

  /// The underlying Ticket NFT Mint account
  #[account(
    init,
    payer = ticket_buyer,
    mint::decimals = 0,
    mint::authority = nft_authority,
    seeds = [
      b"ticket_nft",
      state.key().as_ref(),
      ticket_buyer.key().as_ref(),
      &event_id
    ],
    bump,
  )]
  pub nft: Box<Account<'info, Mint>>,

  /// CHECK: The event nft metadata. We know it's gonna be the correct one because this instruction can only be called
  /// from the Ticket sale program which knows all the seeds to re-create the Pubkey
  #[account()]
  pub event_nft_metadata: AccountInfo<'info>,

  /// CHECK: The metaplex metadata account that will be initialized in the processor
  #[account(
    mut,
    seeds = [PREFIX.as_bytes(), metadata_id.as_ref(), nft.key().as_ref()],
    seeds::program = metadata_id,
    bump,
  )]
  pub metadata: AccountInfo<'info>,

  /// CHECK: The NFT master edition account
  #[account(
    mut,
    seeds = [PREFIX.as_bytes(), metadata_id.as_ref(), nft.key().as_ref(), "edition".as_bytes()],
    seeds::program = metadata_id,
    bump,
  )]
  pub master_edition: AccountInfo<'info>,

  /// The ATA that is a PDA controlled by the Ticket sale program and will be the owner of the Ticket NFT
  /// until the end of the event.
  #[account(
    init,
    payer = ticket_buyer,
    associated_token::mint = nft,
    associated_token::authority = ticket_sale_cpi_authority,
  )]
  pub ticket_nft_ata: Box<Account<'info, TokenAccount>>,

  #[account(
    mut,
    seeds = [b"ticket_sale:cpi_authority", state.ticket_sale_state.as_ref()],
    bump = cpi_authority_bump,
    seeds::program = state.ticket_sale_program,
  )]
  pub ticket_sale_cpi_authority: Signer<'info>,

  /// This is the user that buys the ticket
  #[account(mut)]
  pub ticket_buyer: Signer<'info>,

  associated_token_program: Program<'info, AssociatedToken>,
  pub token_program: Program<'info, Token>,
  /// CHECK: The metadata program
  pub metadata_program: AccountInfo<'info>,
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
}
