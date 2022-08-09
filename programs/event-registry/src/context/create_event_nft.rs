use anchor_lang::prelude::*;
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
  },
};

/// The reason we use this separately from the create_event ix is due to the tx size limitation.
/// There is a proposal to fix this (https://docs.solana.com/proposals/transactions-v2) but in the meantime 
/// the solution is to split the tx into multiple and track the process via on-chain state.
/// This is the approach used by the BPF loader program for deploying Solana programs.
#[derive(Accounts)]
#[instruction(event_id: [u8; 32])]
pub struct CreateEventNft<'info> {
  #[account(mut)]
  pub state: Box<Account<'info, State>>,

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
    seeds = [b"event_nft", state.key().as_ref(), &event_id],
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

  #[account(mut)]
  pub event_organizer: Signer<'info>,
  
  pub token_program: Program<'info, Token>,
  associated_token_program: Program<'info, AssociatedToken>,
  /// CHECK: The metadata program
  pub metadata_program: AccountInfo<'info>,
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
}
