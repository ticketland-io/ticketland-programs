use anchor_lang::prelude::*;

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
