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
    event::{Event, SPACE_MARGIN},
  },
};

#[derive(Accounts)]
pub struct CreateMint<'info> {
  #[account()]
  pub state: Account<'info, State>,

  // The newly created event 
  #[account(
    init,
    payer = creator,
    space = 8 + size_of::<Event>() + SPACE_MARGIN,
    seeds = [b"event", state.key().as_ref(), &state.n_events.to_string().as_ref()],
    bump
  )]
  pub event: Account<'info, Event>,

  #[account(
    init,
    payer = creator,
    mint::decimals = 0,
    mint::authority = event_nft_authority,
    seeds = [b"event_nft", state.key().as_ref()],
    bump,
  )]
  pub event_nft: Account<'info, Mint>,

  /// CHECK: The authority of the event nfts
  #[account(
    seeds = [b"event_nft_authority", state.key().as_ref()],
    bump,
  )]
  pub event_nft_authority: AccountInfo<'info>,

  #[account(
    init,
    payer = creator,
    associated_token::mint = event_nft,
    associated_token::authority = event_nft_authority,
  )]
  pub event_nft_authority_ata: Account<'info, TokenAccount>,

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
  pub creator: Signer<'info>,
  
  pub token_program: Program<'info, Token>,
  associated_token_program: Program<'info, AssociatedToken>,
  system_program: Program<'info, System>,
  rent: Sysvar<'info, Rent>,
}
