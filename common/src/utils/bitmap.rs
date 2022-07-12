fn index_to_byte_and_bit(index: u32) -> (usize, usize, usize) {
	let byte = (index as f64 / 8_f64).floor() as usize;
	let bit = index as usize - (byte * 8); // instead of using module index % 8
  let mask = 1 << bit;

  (byte, bit, mask)
}

/// Checks if the value at the given index is true or false
pub fn is_true<const COUNT: usize>(index: u32, bitmap: &[u8; COUNT]) -> bool {
  let (byte, bit, _) = index_to_byte_and_bit(index);

  (bitmap[byte] >> bit) % 2 == 1
} 

pub fn flip_bit<const COUNT: usize>(index: u32, bitmap: &mut [u8; COUNT]) {
  let (byte, _, mask) = index_to_byte_and_bit(index);
  
  bitmap[byte] = (bitmap[byte] as usize ^ mask) as u8;
}
