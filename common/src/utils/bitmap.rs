use std::ops::{Shr, BitXorAssign};

fn index_to_byte_and_bit(index: usize) -> (usize, usize, i32) {
	let byte = (index as f64 / 8_f64).floor() as usize;
	let bit = index % 8;
  let mask: i32 = 1 << bit;

  (byte, bit, mask)
}

/// Checks if the value at the given index is true or false
pub fn is_set<T, const COUNT: usize>(index: usize, bitmap: [T; COUNT]) -> bool 
where 
  T: Sized + Shr<Output = i32> + From<usize> + Copy
{
  let (byte, bit, _) = index_to_byte_and_bit(index);

  (bitmap[byte] >> bit.into()) % 2 == 1
} 

pub fn flip_bit<T, const COUNT: usize>(index: usize, bitmap: &mut [T; COUNT]) 
where 
  T: Sized + BitXorAssign + From<i32> + Copy
{
  let (byte, _, mask) = index_to_byte_and_bit(index);
  
  bitmap[byte] ^= mask.into();
}
