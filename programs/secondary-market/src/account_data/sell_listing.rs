use anchor_lang::prelude::*;

// Additional space in bytes (1kb) we want to allocate for potential future state expansion
pub const SPACE_MARGIN: usize = 1000;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct MarketBump {
  pub market: u8,
}

#[account]
pub struct Market {
  pub id: [u8; 32],
  pub event_id: [u8; 32],

  pub bumps: MarketBump,

  /// percentage fee that will be collected by the event organizer when someone sells the ticket on the secondary maker
  pub organizer_resale_fee: u16,

  /// This is a percentage of the price the ticket was sold in the primary market. One can list a ticket
  /// which will no exceed the given resale cap
  pub resale_cap: u16,
}
