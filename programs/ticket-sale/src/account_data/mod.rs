use common::{
  impl_currency,
  impl_ticket_type,
};

impl_ticket_type!();
impl_currency!();

pub mod state;
pub mod sale;
pub mod event_capacity;
pub mod event;
