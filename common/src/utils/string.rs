/// Strings need to be appended with `\0`s in order to have a deterministic length.
/// This supports the `memcmp` filter  on get program account calls.
/// Pads the string to the desired size with `0u8`s.
/// NOTE: it is assumed that the string's size is never larger than the given size.
pub fn puffed_out_string(s: &str, size: usize) -> String {
  let mut array_of_zeroes = vec![];
  let puff_amount = size - s.len();

  while array_of_zeroes.len() < puff_amount {
    array_of_zeroes.push(0u8);
  }

  s.to_owned() + std::str::from_utf8(&array_of_zeroes).unwrap()
}
