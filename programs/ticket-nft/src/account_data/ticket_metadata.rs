use anchor_lang::prelude::*;

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

  /// The price the ticket is sold for. The reason we need to keep this explicitly is because
  /// the sale type can an auction and thus the Sale account can't tell us what price the ticket was
  /// sold for.
  pub price_sold: u64,

  /// The original owner of the Ticket NFT
  pub owner: Pubkey,

  /// The metaplex metadata associated with this NFT
  pub metadata: Pubkey,
}
