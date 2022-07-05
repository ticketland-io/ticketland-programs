use anchor_lang::prelude::*;

pub const MAX_CURRENCY_SUPPORT: usize = 10;

// Additional space in bytes (5kb) we want to allocate for potential future state expansion
pub const SPACE_MARGIN: usize = 5000;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct InitBumps {
  pub fund_manager: u8,
  pub event_nft_authority: u8,
}

#[account]
#[derive(Default)]
pub struct State {
  pub bumps: InitBumps,
  pub n_events: u64,
  pub deployer: Pubkey,
  pub supported_currencies: Vec<Pubkey>,
}
