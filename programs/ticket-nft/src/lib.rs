pub mod account_data;
pub mod context;
pub mod processors;
pub mod utils;

use anchor_lang::prelude::*;
use crate::{
  context::{
    initialize::*,
		create_ticket::*,
  },
};


declare_id!("599YwRjALAKVj7z9bcBijrYHyNGLTJSjmJTzeyttnEFL");

#[program]
pub mod ticket_nft {
	use super::*;

	/// Initializes the state i.e. instance of a given program
	/// 
	/// # Arguments
	/// 
	/// * `ctx` - The Anchor context holding the accounts
	pub fn initialize(
		ctx: Context<Initialize>,
		ticket_sale_state: Pubkey,
		ticket_sale_program: Pubkey,
	) -> Result<()> {
    processors::initialize::exec(ctx, ticket_sale_state, ticket_sale_program)
	}

	/// Creates a new Ticket NFT. Only the ticket sale contract can call this instruction
	/// 
	/// # Arguments
	/// 
	/// * `ctx` - The Anchor context holding the accounts
	/// * `_cpi_authority_bump` - The bump of the ticket sale cpi authority i.e. the PDA that can call this instruction
	/// * `event_id` - The event id for which this ticket is created
	/// * `seat_index` - The index of the seat
	/// * `sale` - The sale account though which this the ticket was purchased
	/// * `name` - The name that will be attached to the metaplex metadata. This will most likely be the name of the seat
	pub fn create_ticket(
		ctx: Context<CreateTicket>,
		_cpi_authority_bump: u8,
		event_id: [u8; 32],
		seat_index: u32,
		sale: Pubkey,
		price_sold: u64,
		name: String,
	) -> Result<()> {
    processors::create_ticket::exec(
			ctx,
			event_id,
			sale,
			price_sold,
			seat_index,
			name
		)
	}
}
