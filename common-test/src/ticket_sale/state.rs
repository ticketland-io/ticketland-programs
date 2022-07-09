use anchor_lang::{AnchorDeserialize};


pub const MAX_VENUE_CAPACITY: usize = 100_000;

// The original struct used in the Program is a zero_copy and thus it doesn't implement
// AnchorDeserialize. We use a copy-cat here so we can deserialize the raw data.
#[derive(Debug)]
pub struct EventCapacity {
  pub event_id: u64,
  pub is_initialized: bool,
  pub available_tickets: u32,
  pub seats: [u8; MAX_VENUE_CAPACITY],
}

impl AnchorDeserialize for EventCapacity {
  fn deserialize(data: &mut &[u8]) -> std::result::Result<Self, std::io::Error> {
    print!(">>>>>>>>{:?}", data);
    Self::try_from_slice(data)
  }
}
