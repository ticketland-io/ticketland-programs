pub mod account_data;
pub mod context;
pub mod processors;
pub mod utils;

use crate::context::{create_ticket::*, initialize::*, set_secondary_market::*, transfer::*};
use anchor_lang::prelude::*;

declare_id!("599YwRjALAKVj7z9bcBijrYHyNGLTJSjmJTzeyttnEFL");

#[program]
pub mod ticket_nft {
  use super::*;

  /// Initializes the state i.e. instance of a given program
  ///
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  /// * `ticket_sale_state` - The state of the ticket sale program
  /// * `ticket_sale_program` - The program id of the ticket sale program
  pub fn initialize(
    ctx: Context<Initialize>,
    ticket_sale_state: Pubkey,
    ticket_sale_program: Pubkey,
  ) -> Result<()> {
    processors::initialize::exec(ctx, ticket_sale_state, ticket_sale_program)
  }

  /// Stores the secondary market state and program id
  ///
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  /// * `secondary_market_state` - The state of the secondary market program
  /// * `secondary_market_program` - The program id of the secondary market program
  pub fn set_secondary_market(
    ctx: Context<SetSecondaryMarket>,
    secondary_market_state: Pubkey,
    secondary_market_program: Pubkey,
  ) -> Result<()> {
    processors::set_secondary_market::exec(
      ctx,
      secondary_market_state,
      secondary_market_program,
    )
  }

  /// Creates a new Ticket NFT. Only the ticket sale contract can call this instruction
  ///
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  /// * `_cpi_authority_bump` - The bump of the ticket sale cpi authority i.e. the PDA that can call this instruction
  /// * `_ticket_type_index` - The ticket type index the given ticket belongs to; used as seeds for ticket_nft
  /// * `event_id` - The event id for which this ticket is created
  /// * `seat_index` - The index of the seat
  /// * `recipient` - The account that will be the owner of this ticket
  /// * `sale` - The sale account though which this the ticket was purchased
  /// * `price_sold` - The price for which this ticket was sold fors
  /// * `name` - The name that will be attached to the metaplex metadata. This will most likely be the name of the seat
  pub fn create_ticket(
    ctx: Context<CreateTicket>,
    _cpi_authority_bump: u8,
    _ticket_type_index: u8,
    event_id: [u8; 32],
    seat_index: u32,
    recipient: Pubkey,
    sale: Pubkey,
    price_sold: u64,
    name: String,
  ) -> Result<()> {
    processors::create_ticket::exec(
      ctx, event_id, sale, price_sold, seat_index, recipient, name,
    )
  }

  pub fn transfer(ctx: Context<Transfer>, new_owner: Pubkey) -> Result<()> {
    processors::transfer::exec(ctx, new_owner)
  }
}
