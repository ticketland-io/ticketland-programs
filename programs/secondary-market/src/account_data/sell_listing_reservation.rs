use std::ops::{
  Deref,
  DerefMut,
};
use anchor_lang::prelude::*;
use common::{
  account_data::reservation::Reservation,
};

#[account]
pub struct SellListingReservation(pub Reservation);

impl Deref for SellListingReservation {
  type Target = Reservation;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for SellListingReservation {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
