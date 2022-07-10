use anchor_lang::prelude::*;

pub const MAX_CURRENCY_SUPPORT: usize = 10;

// Additional space in bytes (5kb) we want to allocate for potential future state expansion
pub const SPACE_MARGIN: usize = 5000;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct InitBumps {
  pub event_nft_authority: u8,
  pub cpi_authority: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Debug, Clone)]
pub struct Currency {
  pub mint_account: Pubkey,
  // The treasury ata that will be receiving the service fees
  pub treasury_ata: Pubkey,
  pub deposit_amount: u64,

  /// The service fee that will be charged for each ticket sale. This is in range [0, 10_000] or [0%, 100%]
  /// This allows us to be flexible and assign different service fees for different currencies
  pub service_fee: u16,
}

#[account]
#[derive(Default)]
pub struct State {
  pub bumps: InitBumps,
  pub n_events: u64,
  pub seller_fee_basis_points: u16,
  pub cpi_authority: Pubkey,
  pub deployer: Pubkey,
  pub supported_currencies: Vec<Currency>,
}
