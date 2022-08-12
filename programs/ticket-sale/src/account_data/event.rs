use std::ops::{
  Deref,
  DerefMut,
};
use anchor_lang::prelude::*;
use common::impl_common_event;

impl_common_event!();

#[account]
pub struct Event(pub CommonEvent);

impl Deref for Event {
  type Target = CommonEvent;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Event {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
