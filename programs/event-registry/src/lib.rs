pub mod context;
pub mod account_data;
pub mod processors;
pub mod acl;
pub mod utils;

use anchor_lang::prelude::*;
use common::{
	account_data::{
		event_registry_state::Currency,
	},
  state::{
    alias::*,
    ticket_type::TicketType,
  },
};
use crate::{
	context::{
		initialize::*,
		create_event::*,
	},
};

declare_id!("TGfdMZj2HoSwdFR5zUAKr8H72XYJ85GQ7my5yZTHGKE");

#[program]
pub mod event_registry {
use super::*;
	/// Initializes the state i.e. instance of a given program
	/// 
	/// # Arguments
	/// 
	/// * `ctx` - The Anchor context holding the accounts
	/// * `supported_currencies` - The list of currencies the current instant will support. These are the
	/// currencies that will be used to pay for the service fee as well as to lock a given amount in order to be able to 
	/// create a new event
	/// * `service_fee` - The service fee that will be charged for each ticket sale. This is in range [0, 10_000] or [0%, 100%]
	/// * `seller_fee_basis_points` - This will be attached to each event NFT metadata that is created. NFT tickets are
	/// non-transferable. However, the same does not apply for the Event NFTs (i.e. Event NFT Collection). We envision event organizer
	/// to want to sell Event NFTs in the future for charity purposes for example.
	pub fn initialize(
		ctx: Context<Initialize>,
		supported_currencies: Vec<Currency>,
		service_fee: u16,
		seller_fee_basis_points: u16,
	) -> Result<()> {
    processors::initialize::exec(
			ctx,
			supported_currencies,
			service_fee,
			seller_fee_basis_points,
		)
	}

	/// Allows anyone who has deposited the minimum deposit amount for the selected currency to create a new event.
	/// This will create the Event NFT as well.
	/// 
	/// # Arguments
	/// 
	/// * `ctx` - The Anchor context holding the accounts
	/// * `start_time` - The Solana slot that describes the start of the event
	/// * `end_time` - The Solana slot that describes the end of the event
	/// * `sale_start_time` - The Solana slot that describes the start of the ticket sale for each ticket type
	/// * `ticket_types` - The details of each ticket type. An event might have a lot of different ticket types and thus prices
	/// * `name` - The name that will be used in the Event NFT metadata
	/// * `symbol` - The symbol that will be used in the Event NFT metadata
	/// * `uri` - The uti that will be used in the Event NFT metadata
	pub fn create_event(
		ctx: Context<CreateEvent>,
		start_time: Slot,
		end_time: Slot,
		sale_start_time: Vec<Slot>,
		ticket_types: Vec<TicketType>,
		name: String,
		symbol: String,
		uri: String,
	) -> Result<()> {
    processors::create_event::exec(
			ctx,
			start_time,
			end_time,
			sale_start_time,
			ticket_types,
			name,
			symbol,
			uri,
		)
	}
}
