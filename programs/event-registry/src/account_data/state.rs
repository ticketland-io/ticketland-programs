use anchor_lang::prelude::*;
use common::{
  account_data::event_registry_state::{State as EventRegistryState},
};

pub const MAX_CURRENCY_SUPPORT: usize = 10;

// Additional space in bytes (5kb) we want to allocate for potential future state expansion
pub const SPACE_MARGIN: usize = 5000;

#[account]
pub struct State(pub EventRegistryState);
