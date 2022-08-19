use anchor_lang::prelude::*;
use std::mem::size_of;
use anchor_spl::{
  token::{Mint, Token, TokenAccount},
  associated_token::AssociatedToken,
};
use crate::{
  account_data::{
    state::*,
    ticket_metadata::{TicketMetadata, SPACE_MARGIN},
  },
};

#[derive(Accounts)]
#[instruction(ticket_owner: Pubkey, event_id: [u8; 32], seat_index: u32)]
pub struct Transfer<'info> {
  #[account()]
  pub state: Box<Account<'info, State>>,

  /// The newly created Ticket Metadata 
  #[account(
    seeds = [
      b"ticket_metadata",
      state.key().as_ref(),
      ticket_nft.key().as_ref(),
    ],
    bump
  )]
  pub ticket_metadata: Box<Account<'info, TicketMetadata>>,

  /// The underlying Ticket NFT Mint account
  #[account(
    seeds = [
      b"ticket_nft",
      state.key().as_ref(),
      ticket_owner.as_ref(),
      seat_index.to_string().as_ref(),
      &event_id
    ],
    bump,
  )]
  pub ticket_nft: Box<Account<'info, Mint>>,
}
