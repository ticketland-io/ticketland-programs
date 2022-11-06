use anchor_lang::prelude::*;
use common::state::alias::Slot;
use crate::{
  context::reserve_seat::*,
};

pub fn exec(
  ctx: Context<ReserveSeat>,
  duration: Slot,
  recipient: Pubkey,
) -> Result<()> {
  let seat_reservation = &mut ctx.accounts.seat_reservation;
  
  seat_reservation.valid_until = Clock::get().unwrap().slot + duration;
  seat_reservation.recipient = recipient;

  Ok(())
}
