pub mod account_data;
pub mod context;
pub mod processors;
pub mod utils;

use anchor_lang::prelude::*;
use crate::{
  context::{
    initialize::*,
		create_market::*,
		create_sell_listing::*,
		create_buy_listing::*,
  },
};

declare_id!("ECRCx1XuhFC1DatvsvMu6nwrHQzMo3h41X2vKdvD7S5f");

#[program]
pub mod secondary_market {
  use crate::context::create_buy_listing::CreateBuyListing;

use super::*;

	/// Initializes the state i.e. instance of a given program
	/// 
	/// # Arguments
	/// 
	/// * `ctx` - The Anchor context holding the accounts
	/// * `event_registry_state` - The state of the event registry program
	/// * `event_registry_program` - The id of the event registry program
	/// * `ticket_sale_state` - The state of the ticket sale program
	/// * `ticket_sale_program` - The program id of the ticket sale program
	/// * `ticket_nft_state` - The state of the ticket nft program
	/// * `ticket_nft_program` - The program id of the ticket nft program
	/// * `protocol_fee` - The ticketland protocol fees of every resale
	pub fn initialize(
		ctx: Context<Initialize>,
		event_registry_state: Pubkey,
		event_registry_program: Pubkey,
		ticket_sale_state: Pubkey,
		ticket_sale_program: Pubkey,
		ticket_nft_state: Pubkey,
		ticket_nft_program: Pubkey,
		protocol_fee: u16,
	) -> Result<()> {
    processors::initialize::exec(
			ctx, 
			event_registry_state,
			event_registry_program,
			ticket_sale_state,
			ticket_sale_program,
			ticket_nft_state,
			ticket_nft_program,
			protocol_fee
		)
	}


	/// Allows the event organizer of the given event id to create a new secondary market
	/// 
	/// # Arguments
	/// 
	/// * `ctx` - The Anchor context holding the accounts
	/// * `market_id` - A unique id for this market
	/// * `event_id` - The event id for which the secondary market is created for
	/// * `organizer_resale_fee` - The percentage of the resale price the event organizer will collect
	/// * `resale_cap` - The max price a ticket can be sold on the secondary market. That will be
	///                  (ticket_matadata.price_sold * (10_000 + resale_cap)) / 10_000
	pub fn create_market(
		ctx: Context<CreateMarket>,
		market_id: [u8; 32],
		event_id: [u8; 32],
		organizer_resale_fee: u16,
		resale_cap: u16,
	) -> Result<()> {
		processors::create_market::exec(
			ctx, 
			market_id,
			event_id,
			organizer_resale_fee,
			resale_cap,
		)
	}

	/// Allows the ticket owner to list the ticket for sale
	/// 
	/// # Arguments
	/// 
	/// * `ctx` - The Anchor context holding the accounts
	/// * `market_id` - A unique id for this market
	/// * `event_id` - The event id for which the secondary market is created for
	/// * `ticket_nft` - The pubic key of the ticket NFT Mint account. The processor will have to check that it belongs to the
	///                  signer of this tx
	/// * `ask_price` - The  price a ticket will be sold. It must be
	///                 lower than (ticket_matadata.price_sold * (10_000 + resale_cap)) / 10_000
	pub fn create_sell_listing(
		ctx: Context<CreateSellListing>,
		_ticket_nft: Pubkey,
		market_id: [u8; 32],
		event_id: [u8; 32],
		ask_price: u64,
	) -> Result<()> {
		processors::create_sell_listing::exec(
			ctx, 
			market_id,
			event_id,
			ask_price,
		)
	}

	/// Allows anyone to create a buy listing that is to publish an interest to buy a ticket at a given price
	/// 
	/// # Arguments
	/// 
	/// * `ctx` - The Anchor context holding the accounts
	/// * `market_id` - A unique id for this market
	/// * `event_id` - The event id for which the secondary market is created for
	/// * `ticket_nft` - The pubic key of the ticket NFT Mint account. The processor will have to check that it belongs to the
	///                  signer of this tx
	/// * `bid_price` - The  price at which the user is willing to buy a ticket. Note that during the settlemt the following must be true
	///                 bid_price <= (ticket_matadata.price_sold * (10_000 + resale_cap)) / 10_000
	pub fn create_buy_listing(
		ctx: Context<CreateBuyListing>,
		market_id: [u8; 32],
		event_id: [u8; 32],
		bid_price: u64,
	) -> Result<()> {
		processors::create_buy_listing::exec(
			ctx, 
			market_id,
			bid_price,
		)
	}
}
