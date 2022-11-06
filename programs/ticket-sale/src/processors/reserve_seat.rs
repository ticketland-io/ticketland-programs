use anchor_lang::prelude::*;
use common::state::alias::Slot;
use crate::{
  context::reserve_seat::*,
};

pub fn exec(
  ctx: Context<ReserveSeat>,
  valid_until: Slot,
  recipient: Pubkey,
) -> Result<()> {
  let seat_reservation = &mut ctx.accounts.seat_reservation;
  
  seat_reservation.valid_until = valid_until;
  seat_reservation.recipient = recipient;

  Ok(())
}
