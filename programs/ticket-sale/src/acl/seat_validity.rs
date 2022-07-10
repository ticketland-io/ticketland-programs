use anchor_lang::{
  prelude::*,
  solana_program::keccak::hashv,
};
use crate::utils::program_error::ErrorCode;
use common::crypto::mt;

pub fn verify<'info>(
  merkle_root: [u8; 32],
  merkle_proof: Vec<[u8; 32]>,
  seat_index: u32,
  seat_name: String,
) -> Result<()> {
  // Create the Leaf which is hashv(seat_index || "." || seat_name)
  let leaf = hashv(&[
    seat_index.to_string().as_ref(),
    b".",
    seat_name.to_string().as_ref()
  ]).0;

  require!(mt::verify(merkle_proof, merkle_root, leaf,), ErrorCode::InvalidProof);

  Ok(())
}
