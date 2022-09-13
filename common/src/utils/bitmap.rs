use spl_math::precise_number::PreciseNumber;

fn index_to_byte_and_bit(index: u32) -> (usize, usize, usize) {
  let byte = PreciseNumber::new(index.into())
  .unwrap()
  .checked_div(&PreciseNumber::new(8).unwrap())
  .unwrap()
  .floor()
  .unwrap()
  .to_imprecise()
  .unwrap() as usize;

	let bit = index as usize - (byte * 8); // instead of using module index % 8
  let mask = 1 << bit;

  (byte, bit, mask)
}

pub fn count_to_len(index: u32) -> usize {
  PreciseNumber::new(index.into())
  .unwrap()
  .checked_div(&PreciseNumber::new(8).unwrap())
  .unwrap()
  .ceiling()
  .unwrap()
  .to_imprecise()
  .unwrap() as usize
}


/// Checks if the value at the given index is true or false
pub fn is_set(index: u32, bitmap: &Vec<u8>) -> bool {
  let (byte, bit, _) = index_to_byte_and_bit(index);

  (bitmap[byte] >> bit) % 2 == 1
} 

pub fn flip_bit(index: u32, bitmap: &mut Vec<u8>) {
  let (byte, _, mask) = index_to_byte_and_bit(index);
  
  bitmap[byte] = (bitmap[byte] as usize ^ mask) as u8;
}
