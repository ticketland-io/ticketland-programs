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
  pub event_id: u64,

  /// The original owner of the Ticket NFT
  pub owner: Pubkey,

  /// The metaplex metadata associated with this NFT
  pub metadata: Pubkey,
}
