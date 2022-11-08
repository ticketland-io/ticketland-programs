use std::ops::{
  Deref,
  DerefMut,
};
use anchor_lang::prelude::*;
use common::{
  account_data::reservation::Reservation,
};

#[account]
pub struct SeatReservation(pub Reservation);

impl Deref for SeatReservation {
  type Target = Reservation;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for SeatReservation {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
