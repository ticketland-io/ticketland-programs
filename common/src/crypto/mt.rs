use anchor_lang::{
  solana_program::keccak::hashv,
};

pub fn get_null_leaf() -> [u8; 32] {
  hashv(&[b"NULL"]).0
}

/// Create the Leaf which is hashv(seat_index || "." || seat_name)
pub fn create_seat_leaf(seat_index: u32, seat_name: &String) -> [u8; 32] {
  hashv(&[
    seat_index.to_string().as_ref(),
    b".",
    seat_name.to_string().as_ref()
  ]).0
}

pub fn verify(proof: Vec<[u8; 32]>, root: [u8; 32], leaf: [u8; 32]) -> bool {
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
