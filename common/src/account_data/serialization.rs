use anchor_lang::{
  prelude::*,
  AccountDeserialize,
  solana_program::account_info::AccountInfo,
};

pub fn deser<T: AccountDeserialize>(account: AccountInfo) -> Result<T> {
  let account_data = account.try_borrow_data()?;
  let mut data_slice: &[u8] = &account_data;
  let data = T::try_deserialize(&mut data_slice)?;
  
  Ok(data)
}

/// Deserializes account data without checking the account discriminator.
pub fn deser_unchecked<T: AccountDeserialize>(account: AccountInfo) -> Result<T> {
  let account_data = account.try_borrow_data()?;
  let mut data_slice: &[u8] = &account_data;
  let data = T::try_deserialize_unchecked(&mut data_slice)?;
  
  Ok(data)
}
