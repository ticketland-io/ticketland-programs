use anchor_lang::{
  prelude::*,
  AccountDeserialize,
  solana_program::account_info::AccountInfo,
};

pub fn deser<T: AccountDeserialize>(
  account: AccountInfo,
  discriminator: &[u8]
) -> Result<T> {
  let account_data = account.try_borrow_data()?;
  let data_slice: &[u8] = &account_data;

  // check the discriminator
  let given_disc: &[u8] = &data_slice[..8];

  if discriminator != given_disc {
    return Err(anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch.into());
  }

  let mut struct_data: &[u8] = &data_slice[8..];
  let data = T::try_deserialize_unchecked(&mut struct_data)?;
  
  Ok(data)
}
