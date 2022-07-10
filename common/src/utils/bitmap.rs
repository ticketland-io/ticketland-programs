use std::ops::{Shr, BitXorAssign};

fn index_to_byte_and_bit(index: u32) -> (u32, u32, u32) {
	let byte = (index as f64 / 8_f64).floor() as u32;
	let bit = index % 8;
  let mask: u32 = 1 << bit;

  (byte, bit, mask)
}

/// Checks if the value at the given index is true or false
pub fn is_true<T, const COUNT: usize>(index: u32, bitmap: &[T; COUNT]) -> bool 
where 
  T: Sized + Shr<Output = T> + Into<u32> + Copy
{
  let (byte, bit, _) = index_to_byte_and_bit(index);

  (bitmap[byte as usize].into() >> bit) % 2 == 1
} 

pub fn flip_bit<T, const COUNT: usize>(index: u32, bitmap: &mut [T; COUNT]) 
where 
  T: Sized + BitXorAssign + Into<u32> + From<u32> + Copy
{
  let (byte, _, mask) = index_to_byte_and_bit(index);
  
  
  bitmap[byte as usize] = (bitmap[byte as usize].into() ^ mask).into();
}
