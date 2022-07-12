use anchor_lang::{
  prelude::*,
};
use crate::utils::program_error::ErrorCode;
use common::crypto::mt::{self, create_seat_leaf};

pub fn verify<'info>(
  merkle_root: [u8; 32],
  merkle_proof: Vec<[u8; 32]>,
  seat_index: u32,
  seat_name: &String,
) -> Result<()> {
  require!(mt::verify(merkle_proof, merkle_root, create_seat_leaf(seat_index, seat_name)), ErrorCode::InvalidProof);

  Ok(())
}
