pub mod context;
pub mod account_data;
pub mod processors;
pub mod utils;

use anchor_lang::prelude::*;
use common::{
	state::{
		currency::*,
		ticket_type::TicketType,
	},
};
use crate::{
	context::{
		initialize::*,
		create_event::*,
		create_event_nft::*,
		update_supported_currencies::*,
		update_event_nft_uri::*,
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
	/// * `uri_update_operators` - The list of accounts that can update the Event NFT metadata uri
	/// * `seller_fee_basis_points` - This will be attached to each event NFT metadata that is created. NFT tickets are
	/// non-transferable. However, the same does not apply for the Event NFTs (i.e. Event NFT Collection). We envision event organizer
	/// to want to sell Event NFTs in the future for charity purposes for example.
	pub fn initialize(
		ctx: Context<Initialize>,
		supported_currencies: Vec<Currency>,
		uri_update_operators: Vec<Pubkey>,
		seller_fee_basis_points: u16,
	) -> Result<()> {
		processors::initialize::exec(
			ctx,
			supported_currencies,
			uri_update_operators,
			seller_fee_basis_points,
		)
	}

	/// Allows anyone who has deposited the minimum deposit amount for the selected currency to create a new event.
	/// This will create the Event NFT as well.
	///
	/// # Arguments
	///
	/// * `ctx` - The Anchor context holding the accounts
	/// * `event_id` - The event id
	/// * `start_time` - The Solana time that describes the start of the event
	/// * `end_time` - The Solana time that describes the end of the event
	/// * `ticket_types` - The details of each ticket type. An event might have a lot of different ticket types and thus prices
	pub fn create_event(
		ctx: Context<CreateEvent>,
		event_id: [u8; 32],
		n_tickets: u32,
		start_time: i64,
		end_time: i64,
		ticket_types: Vec<TicketType>,
	) -> Result<()> {
		processors::create_event::exec(
			ctx,
			event_id,
			n_tickets,
			start_time,
			end_time,
			ticket_types,
		)
	}

	/// This will create the Event NFT for the given event id.
	///
	/// # Arguments
	///
	/// * `ctx` - The Anchor context holding the accounts
	/// * `event_id` - The event id
	/// * `symbol` - The symbol that will be used in the Event NFT metadata
	/// * `uri` - The uti that will be used in the Event NFT metadata
	pub fn create_event_nft(
		ctx: Context<CreateEventNft>,
		_event_id: [u8; 32],
		name: String,
		symbol: String,
		uri: String,
	) -> Result<()> {
		processors::create_event_nft::exec(
			ctx,
			name,
			symbol,
			uri,
		)
	}

	/// Allows the deployer to update the supported currencies
	///
	/// # Arguments
	///
	/// * `ctx` - The Anchor context holding the accounts
	/// * `supported_currencies` - The list of currencies the current instant will support. These are the
	/// currencies that will be used to pay for the service fee as well as to lock a given amount in order to be able to
	/// create a new event
	pub fn update_supported_currencies(
		ctx: Context<UpdateSupportedCurrencies>,
		supported_currencies: Vec<Currency>,
	) -> Result<()> {
		processors::update_supported_currencies::exec(
			ctx,
			supported_currencies,
		)
	}

	/// Allows one of the uri update operators to update the metadata uri of the given event_nft
	///
	/// # Arguments
	///
	/// * `ctx` - The Anchor context holding the accounts
	/// * `event_nft` - The event nft whose metadata uri will be updated
	/// * `new_uri` - The new uri
	pub fn update_event_nft_uri(
		ctx: Context<UpdateEventNftUri>,
		_event_nft: Pubkey,
		new_uri: String,
	) -> Result<()> {
		processors::update_event_nft_uri::exec(
			ctx,
			new_uri,
		)
	}
}
