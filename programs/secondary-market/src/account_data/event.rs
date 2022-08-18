use std::ops::{
  Deref,
  DerefMut,
};
use anchor_lang::prelude::*;
use common::{
  account_data::event::{Event as CommonEvent},
};


// We could import this from the event registry program but that would create a cyclic dependency. Thus we have to
// essentially create a local replica of the same struct
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
