use anchor_lang::prelude::*;

pub const MAX_CURRENCY_SUPPORT: usize = 10;

// Additional space in bytes (5kb) we want to allocate for potential future state expansion
pub const SPACE_MARGIN: usize = 5000;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct InitBumps {
  pub fund_manager: u8,
  pub event_nft_authority: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Currency {
  pub mint_account: Pubkey,
  pub deposit_amount: u64,
}

#[account]
#[derive(Default)]
pub struct State {
  pub bumps: InitBumps,
  pub n_events: u64,
  pub service_fee: u16,
  pub seller_fee_basis_points: u16,
  pub deployer: Pubkey,
  pub supported_currencies: Vec<Currency>,
}
