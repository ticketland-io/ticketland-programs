use anchor_lang::prelude::*;
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
pub struct Purchase<'info> {
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
    bump
  )]
  pub sale: Account<'info, Sale>,

  /// CHECK: The account that will hold the seats bitmap
  #[account(
    mut,
    constraint = EventCapacity::owner() == ID @ ErrorCode::NotOwnedByThisProgram,
  )]
  pub event_capacity: AccountLoader<'info, EventCapacity>,
}
