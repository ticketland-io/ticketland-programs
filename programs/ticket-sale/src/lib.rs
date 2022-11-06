pub mod account_data;
pub mod acl;
pub mod context;
pub mod processors;
pub mod utils;

use anchor_lang::prelude::*;
use common::state::alias::Slot;
use crate::{
  context::{
    initialize::*,
		init_event_capacity::*,
		create_sale::*,
		fixed_price_purchase::*,
		free_purchase::*,
    operator_purchase::*,
		verify_seat::*,
    reserve_seat::*,
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
  /// * `event_registry_state` - The event registry program state
  /// * `event_registry_program` - The event registry program address
  /// * `mint_operators` - The list of all operators that can mint tickets
	pub fn initialize(
		ctx: Context<Initialize>,
		treasury: Pubkey,
		event_registry_state: Pubkey,
		event_registry_program: Pubkey,
    mint_operators: Vec<Pubkey>,
	) -> Result<()> {
    processors::initialize::exec(ctx, treasury, event_registry_state, event_registry_program, mint_operators)
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
		event_id: [u8; 32],
		n_tickets: u32,
	) -> Result<()> {
    processors::init_event_capacity::exec(ctx, event_id, n_tickets)
	}

	/// Creates a new sale
	/// 
	/// # Arguments
	/// 
	/// * `ctx` - The Anchor context holding the accounts
	/// * `ticket_type_index` - A unique index that will differentiate multiple sales of one single event. This is useful to 
	/// create unique sale PDAs
	/// * `event_id` - The event id for which this sale is created for
	pub fn create_sale(
		ctx: Context<CreateSale>,
		ticket_type_index: u8,
		event_id: [u8; 32],
	) -> Result<()> {
		processors::create_sale::exec(
			ctx,
			ticket_type_index,
			event_id,
		)
	}

	/// This is the first ix user should call before buying a ticket. The reason we use this is purely due to Solana
	/// Tx size limitation which is currently 1232 bytes. This causes an issue when we have a lot of tickets for an event
	/// which results in longer merkle proofs which cause the tx to exceed the aforementioned size limit.
	/// Solana will ultimately implement Tx v2 https://docs.solana.com/proposals/transactions-v2 which will allow to circumvent
	/// such limitations. However, for the time being we need to use this multiple-step process.
	/// 
	/// # Arguments
	/// 
	/// * `ctx` - The Anchor context holding the accounts
	/// * `seat_index` - The index of the seat which is the bitmap index i.e. a monotonic value
	/// * `seat_name` - Arbitrary name of the seat a defined in the leaves of the MT
	/// * `merkle_proof` - The proof that will make sure that user does not buy a seat of a higher type by paying lower amount
	pub fn verify_seat(
		ctx: Context<VerifySeat>,
		seat_index: u32,
		seat_name: String,
		merkle_proof: Vec<[u8; 32]>,
	) -> Result<()> {
		processors::verify_seat::exec(
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
	pub fn fixed_price_purchase(
		ctx: Context<FixedPricePurchase>,
		seat_index: u32,
		seat_name: String,
	) -> Result<()> {
		processors::fixed_price_purchase::exec(
			ctx,
			seat_index,
			seat_name,
		)
	}

	/// Allows user to purchase a ticket for free
	/// 
	/// # Arguments
	/// 
	/// * `ctx` - The Anchor context holding the accounts
	/// * `seat_index` - The index of the seat which is the bitmap index i.e. a monotonic value
	/// * `seat_name` - Arbitrary name of the seat a defined in the leaves of the MT
	pub fn free_purchase(
		ctx: Context<FreePurchase>,
		seat_index: u32,
		seat_name: String,
	) -> Result<()> {
		processors::free_purchase::exec(
			ctx,
			seat_index,
			seat_name,
		)
	}

  /// Allows a mint operator to mint a new ticket to the recipient account
	/// 
	/// # Arguments
	/// 
	/// * `ctx` - The Anchor context holding the accounts
	/// * `seat_index` - The index of the seat which is the bitmap index i.e. a monotonic value
	/// * `seat_name` - Arbitrary name of the seat a defined in the leaves of the MT
  /// * `recipient` - The recipient account that will receive the ticket NFT
	pub fn operator_purchase(
		ctx: Context<OperatorPurchase>,
		seat_index: u32,
		seat_name: String,
    recipient: Pubkey,
	) -> Result<()> {
		processors::operator_purchase::exec(
			ctx,
			seat_index,
			seat_name,
      recipient,
		)
	}

  /// This can only be called by a ticketland operator and it reserves the given seat until the given slot.
  /// This will reserve the seat so no other script can purchase it. The OperatorPurchase Ix will ultimately 
	/// 
	/// # Arguments
	/// 
	/// * `ctx` - The Anchor context holding the accounts
	/// * `seat_index` - The index of the seat which is the bitmap index i.e. a monotonic value
	/// * `seat_name` - Arbitrary name of the seat a defined in the leaves of the MT
  /// * `duration` - The duration in Slot that his seat will be reserved for
	/// * `recipient` - The utlimate recipient of the ticket nft when it will be minted by the operator
  pub fn reserve_seat(
    ctx: Context<ReserveSeat>,
		_seat_index: u32,
		_seat_name: String,
    duration: Slot,
    recipient: Pubkey,
  ) -> Result<()> {
    processors::reserve_seat::exec(ctx, duration, recipient)
  }
}
