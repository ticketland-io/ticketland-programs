use anchor_lang::prelude::*;
use anchor_metaplex::{
  mpl_token_metadata::{
    ID as metadata_id,
    state::{PREFIX},
  }
};
use crate::{
  utils::program_error::ErrorCode,
  account_data::{
    state::*,
  },
};

#[derive(Accounts)]
#[instruction(event_nft: Pubkey)]
pub struct UpdateEventNftUri<'info> {
  #[account(mut)]
  pub state: Box<Account<'info, State>>,

  /// CHECK: The authority of the event nfts
  #[account(
    mut,
    seeds = [b"event_nft_authority", state.key().as_ref()],
    bump = state.bumps.event_nft_authority,
  )]
  pub event_nft_authority: AccountInfo<'info>,

  /// CHECK: The metadata account that will be initialized in the processor
  #[account(
    mut,
    seeds = [PREFIX.as_bytes(), metadata_id.as_ref(), event_nft.as_ref()],
    seeds::program = metadata_id,
    bump,
  )]
  pub metadata: AccountInfo<'info>,

  #[account(
    mut,
    constraint = state.uri_update_operators.iter().any(|u| u.key() == uri_update_operator.key()) @ ErrorCode::OnlyUriUpdateOperator,
  )]
  pub uri_update_operator: Signer<'info>,

  /// CHECK: The metadata program
  pub metadata_program: AccountInfo<'info>,
}
