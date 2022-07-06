mod account_data;
mod context;
mod processors;
mod utils;

use anchor_lang::prelude::*;
use common::{
	state::ticket_type::TicketType,
};
use crate::{
  context::{
    initialize::*,
		create_sale::*,
  },
};

declare_id!("6banhWF9WKQk26NtgX6TqHNmAKgvX9aJmgaBvPmYCCK3");

#[program]
pub mod ticket_sale {
	use super::*;

	/// Initializes the state i.e. instance of a given program
	/// 
	/// # Arguments
	/// 
	/// * `ctx` - The Anchor context holding the accounts
	pub fn initialize(
		ctx: Context<Initialize>,
	) -> Result<()> {
    processors::initialize::exec(ctx)
	}

	/// Creates a new sale
	/// 
	/// # Arguments
	/// 
	/// * `ctx` - The Anchor context holding the accounts
	/// * `ticket_type_index` - A unique index that will differentiate multiple sales of one single event. This is useful to 
	/// create unique sale PDAs
	/// * `cpi_authority_bump` - The bump of the cpi authority account created in the event registry program
	/// * `event_id` - The event id for which this sale is created for
	/// * `ticket_type` - The ticket type that will be sold during this sale
	pub fn create_sale(
		ctx: Context<CreateSale>,
		ticket_type_index: u8,
		_cpi_authority_bump: u8,
		event_id: u64,
		ticket_type: TicketType,
	) -> Result<()> {
		processors::create_sale::exec(
			ctx,
			ticket_type_index,
			event_id,
			ticket_type
		)
	}
}
