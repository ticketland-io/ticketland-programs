use anchor_lang::prelude::*;
use common::state::alias::Slot;
use crate::{
  context::reserve_sell_listing::*,
};

pub fn exec(
  ctx: Context<ReserveSellListing>,
  duration: Slot,
  recipient: Pubkey,
) -> Result<()> {
  let sell_listing_reservation = &mut ctx.accounts.sell_listing_reservation;
  
  sell_listing_reservation.valid_until = Clock::get().unwrap().slot + duration;
  sell_listing_reservation.recipient = recipient;

  Ok(())
}
