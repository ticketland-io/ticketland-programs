use anchor_lang::prelude::*;

// Additional space in bytes (1kb) we want to allocate for potential future state expansion
pub const SPACE_MARGIN: usize = 1000;


/// A wrapper around the Metaplex metadata that includes additional custom data related to our
/// ticketing system.
#[account]
pub struct TicketMetadata {
  /// Indicated if the owner of this ticket attended the event
  pub attended: bool,
  
  /// The id of the event which this ticket belongs to
  pub event_id: [u8; 32],

  /// The seat index of this ticket
  pub seat_index: u32,

  /// The primary sale account containing information about how the sale of the ticket.
  /// This is useful as we can retrieve information such as ticket type etc.
  pub sale: Pubkey,

  /// The original owner of the Ticket NFT
  pub owner: Pubkey,

  /// The metaplex metadata associated with this NFT
  pub metadata: Pubkey,
}
