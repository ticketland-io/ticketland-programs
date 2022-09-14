use anchor_lang::prelude::*;
use crate::{
  account_data::{
    state::*,
    sale::*,
  },
};

#[derive(Accounts)]
#[instruction(seat_index: u32)]
pub struct VerifySeat<'info> {
  #[account(mut)]
  pub state: Box<Account<'info, State>>,

  #[account(
    mut,
    seeds = [
      b"sale",
      state.key().as_ref(),
      sale.ticket_type_index.to_string().as_ref(),
      &sale.event_id
    ],
    bump = sale.bump,
  )]
  pub sale: Box<Account<'info, Sale>>,
}
