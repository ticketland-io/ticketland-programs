use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct InitBumps {
  pub event_nft_authority: u8,
  pub cpi_authority: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Currency {
  pub mint_account: Pubkey,
  pub deposit_amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone)]
pub struct State {
  pub bumps: InitBumps,
  pub n_events: u64,
  pub service_fee: u16,
  pub seller_fee_basis_points: u16,
  pub deployer: Pubkey,
  pub supported_currencies: Vec<Currency>,
}
