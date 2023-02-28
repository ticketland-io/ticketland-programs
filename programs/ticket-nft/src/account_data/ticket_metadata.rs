use anchor_lang::prelude::*;

pub const MAX_NAME_LENGTH: usize = 32;

// Symbol and URI do not have an explicit max length in our program. This is because we copy those values from the
// Event NFT which is a Metaplex metadata account that does enforce max lengths already. In essence, it is indirectly applied
pub const MAX_SYMBOL_LENGTH: usize = 10;
pub const MAX_URI_LENGTH: usize = 200;

pub const ADDITIONAL_SIZE: usize = MAX_NAME_LENGTH
  + MAX_SYMBOL_LENGTH
  + MAX_URI_LENGTH;

/// A wrapper around the Metaplex metadata that includes additional custom data related to our
/// ticketing system.
#[account]
pub struct TicketMetadata {
  /// The actual token Mint account
  pub mint: Pubkey,

  /// The Event NFT that this NFT belongs to
  pub collection: Pubkey,

  /// The name of the asset
  pub name: String,

  /// Indicated if the owner of this ticket attended the event
  pub attended: bool,
  
  /// The id of the event which this ticket belongs to
  pub event_id: [u8; 32],

  /// The seat index of this ticket
  pub seat_index: u32,

  /// The primary sale account containing information about how the sale of the ticket.
  /// This is useful as we can retrieve information such as ticket type etc.
  pub sale: Pubkey,

  /// The price the ticket is sold for. The reason we need to keep this explicitly is because
  /// the sale type can an auction and thus the Sale account can't tell us what price the ticket was
  /// sold for.
  pub price_sold: u64,

  /// The original owner of the Ticket NFT
  pub owner: Pubkey,
}
