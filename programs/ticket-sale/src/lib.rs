mod account_data;
mod context;
mod processors;
mod utils;

use anchor_lang::prelude::*;

use crate::{
  context::{
    initialize::*,
  },
};

declare_id!("6banhWF9WKQk26NtgX6TqHNmAKgvX9aJmgaBvPmYCCK3");

#[program]
pub mod ticket_sale {
	use super::*;

	pub fn initialize(
		ctx: Context<Initialize>,
		_event_registry_state: Pubkey,
	) -> Result<()> {
    processors::initialize::exec(
			ctx,
		)
	}
}
