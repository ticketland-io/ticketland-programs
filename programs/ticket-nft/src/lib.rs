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
	) -> Result<()> {
    processors::initialize::exec(ctx)
	}

	/// Creates a new Ticket NFT. Only the ticket sale contract can call this instruction
	/// 
	/// # Arguments
	/// 
	/// * `ctx` - The Anchor context holding the accounts
	/// * `_cpi_authority_bump` - The bump of the ticket sale cpi authority i.e. the PDA that can call this instruction
	/// * `event_id` - The event id for which this ticket is created
	/// * `name` - The name that will be attached to the metaplex metadata. This will most likely be the name of the seat
	pub fn create_ticket(
		ctx: Context<CreateTicket>,
		_cpi_authority_bump: u8,
		event_id: [u8; 32],
		name: String,
	) -> Result<()> {
    processors::create_ticket::exec(ctx, event_id, name)
	}
}
