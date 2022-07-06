use anchor_lang::prelude::*;
use anchor_lang::{
  solana_program::keccak::hashv,
};
use crate::{
  program_error::{ErrorCode},
};

fn verify(proof: Vec<[u8; 32]>, root: [u8; 32], leaf: [u8; 32]) -> bool {
  let mut computed_hash = leaf;

  for proof_element in proof.into_iter() {
    if computed_hash <= proof_element {
      // Hash(current computed hash + current element of the proof)
      computed_hash = hashv(&[&computed_hash, &proof_element]).0;
    } else {
      // Hash(current element of the proof + current computed hash)
      computed_hash = hashv(&[&proof_element, &computed_hash]).0;
    }
  }

  // Check if the computed hash (root) is equal to the provided root
  computed_hash == root
}

pub fn can_mint<'info>(
  merkle_proof: Vec<[u8; 32]>,
  token_id: u32,
  name: u32,
  merkle_root: [u8; 32],
  user: &Signer<'info>,
) -> Result<()> {
  let leaf = hashv(&[
    user.key().as_ref(),
    token_id.to_string().as_ref(),
    name.to_string().as_ref()
  ]).0;
  require!(verify(merkle_proof, merkle_root, leaf), ErrorCode::InvalidMerkleProof);
  
  Ok(())
}
