use anchor_lang::prelude::*;
use std::mem::size_of;
use crate::{
  account_data::{
    state::*,
    sale::*,
    seat_reservation::*,
  },
  utils::program_error::ErrorCode,
};

#[derive(Accounts)]
#[instruction(seat_index: u32, seat_name: String)]
pub struct ReserveSeat<'info> {
  #[account()]
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
    init_if_needed,
    payer = operator,
    space = 8 + size_of::<SeatReservation>(),
    seeds = [
      b"seat_reservation",
      sale.key().as_ref(),
      seat_index.to_string().as_ref(),
      seat_name.as_ref(),
    ],
    bump,
  )]
  pub seat_reservation: Account<'info, SeatReservation>,

  #[account(
    mut,
    constraint = state.mint_operators.iter().any(|m| *m == operator.key()) @ ErrorCode::OnlyMintOperator,
  )]
  pub operator: Signer<'info>,

  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
}
