use anchor_lang::prelude::*;
use common::{
  state::currency::*,
};

pub const MAX_CURRENCY_SUPPORT: usize = 10;
pub const MAX_URI_UPDATE_OPERATORS: usize = 10;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct InitBumps {
  pub event_nft_authority: u8,
  pub cpi_authority: u8,
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
  pub uri_update_operators: Vec<Pubkey>,
}
