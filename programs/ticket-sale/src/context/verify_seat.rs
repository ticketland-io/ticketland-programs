use anchor_lang::prelude::*;
use std::mem::size_of;
use crate::{
  account_data::{
    state::*,
    sale::*,
    seat_verification::*,
  },
};

#[derive(Accounts)]
#[instruction(seat_index: u32, seat_name: String)]
pub struct VerifySeat<'info> {
  #[account(mut)]
  pub state: Account<'info, State>,

  #[account(
    seeds = [
      b"sale",
      state.key().as_ref(),
      sale.ticket_type_index.to_string().as_ref(),
      &sale.event_id
    ],
    bump = sale.bump,
  )]
  pub sale: Account<'info, Sale>,

  #[account(
    init,
    payer = ticket_buyer,
    space = 8 + size_of::<SeatVerification>(),
    seeds = [
      b"seat_verification",
      state.key().as_ref(),
      seat_index.to_string().as_ref(),
      seat_name.as_ref(),
    ],
    bump,
  )]
  pub seat_verification: Account<'info, SeatVerification>,

  #[account(mut)]
  pub ticket_buyer: Signer<'info>,

  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
}
