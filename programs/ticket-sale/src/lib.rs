pub mod account_data;
pub mod acl;
pub mod context;
pub mod processors;
pub mod utils;

use anchor_lang::prelude::*;
use common::{
	state::{
		ticket_type::TicketType,
	},
};
use crate::{
  context::{
    initialize::*,
		init_event_capacity::*,
		create_sale::*,
		fixed_price_purchase::*,
		free_purchase::*,
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
	/// * `treasury` - The ticketland.io treasury address
	pub fn initialize(
		ctx: Context<Initialize>,
		treasury: Pubkey,
	) -> Result<()> {
    processors::initialize::exec(ctx, treasury)
	}

	/// Initializes the event capacity account for a single event. This account will be used by all the
	/// ticket sale when a purchase happens. This will be a CPI call from the event registry which will be
	/// called before any ticket sale is created.
	/// 
	/// # Arguments
	/// 
	/// * `ctx` - The Anchor context holding the accounts
	/// * `cpi_authority_bump` - The bump of the cpi authority account created in the event registry program
	/// * `event_id` - The event id for which this sale is created for
	/// * `n_tickets` - Total tickets for the given event
	pub fn init_event_capacity(
		ctx: Context<InitEventCapacity>,
		_cpi_authority_bump: u8,
		event_id: u64,
		n_tickets: u32,
	) -> Result<()> {
    processors::init_event_capacity::exec(ctx, event_id, n_tickets)
	}

	/// Creates a new sale
	/// 
	/// # Arguments
	/// 
	/// * `ctx` - The Anchor context holding the accounts
	/// * `cpi_authority_bump` - The bump of the cpi authority account created in the event registry program
	/// * `ticket_type_index` - A unique index that will differentiate multiple sales of one single event. This is useful to 
	/// create unique sale PDAs
	/// * `event_id` - The event id for which this sale is created for
	/// * `ticket_type` - The ticket type that will be sold during this sale
	pub fn create_sale(
		ctx: Context<CreateSale>,
		_cpi_authority_bump: u8,
		ticket_type_index: u8,
		event_id: u64,
		ticket_type: TicketType,
	) -> Result<()> {
		processors::create_sale::exec(
			ctx,
			ticket_type_index,
			event_id,
			ticket_type,
		)
	}

	/// Allows user to purchase a ticket on a fixed price
	/// 
	/// # Arguments
	/// 
	/// * `ctx` - The Anchor context holding the accounts
	/// * `seat_index` - The index of the seat which is the bitmap index i.e. a monotonic value
	/// * `seat_name` - Arbitrary name of the seat a defined in the leaves of the MT
	/// * `merkle_proof` - The proof that will make sure that user does not buy a seat of a higher type by paying lower amount
	pub fn fixed_price_purchase(
		ctx: Context<FixedPricePurchase>,
		seat_index: u32,
		seat_name: String,
		merkle_proof: Vec<[u8; 32]>,
	) -> Result<()> {
		processors::fixed_price_purchase::exec(
			ctx,
			seat_index,
			seat_name,
			merkle_proof,
		)
	}

	/// Allows user to purchase a ticket on a fixed price
	/// 
	/// # Arguments
	/// 
	/// * `ctx` - The Anchor context holding the accounts
	/// * `seat_index` - The index of the seat which is the bitmap index i.e. a monotonic value
	/// * `seat_name` - Arbitrary name of the seat a defined in the leaves of the MT
	/// * `merkle_proof` - The proof that will make sure that user does not buy a seat of a higher type by paying lower amount
	pub fn free_purchase(
		ctx: Context<FreePurchase>,
		seat_index: u32,
		seat_name: String,
		merkle_proof: Vec<[u8; 32]>,
	) -> Result<()> {
		processors::free_purchase::exec(
			ctx,
			seat_index,
			seat_name,
			merkle_proof,
		)
	}
}
