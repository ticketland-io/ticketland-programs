use anchor_lang::prelude::*;
use anchor_safe_math::SafeMath;

#[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Default, Copy, Debug, Clone)]
pub struct Currency {
  pub mint_account: Pubkey,
  // The treasury ata that will be receiving the service fees
  pub treasury_ata: Pubkey,
  pub deposit_amount: u64,

  /// The service fee that will be charged for each ticket sale. This is in range [0, 10_000] or [0%, 100%]
  /// This allows us to be flexible and assign different service fees for different currencies
  pub service_fee: u16,
}

impl Currency {
  pub fn calc_fee(&self, amount: u64) -> Result<(u64, u64)> {
    let service_fee = amount
      .safe_mul(self.service_fee as u64)?
      .safe_div(10_000)?;
  
    let event_organizer_amount = amount.safe_sub(service_fee)?;
    
    Ok((event_organizer_amount, service_fee))
  }
}
