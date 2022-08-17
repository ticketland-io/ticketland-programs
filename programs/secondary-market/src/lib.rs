pub mod account_data;
pub mod context;
pub mod processors;
pub mod utils;

use anchor_lang::prelude::*;
use crate::{
  context::{
    initialize::*,
  },
};

declare_id!("ECRCx1XuhFC1DatvsvMu6nwrHQzMo3h41X2vKdvD7S5f");

#[program]
pub mod secondary_market {
  use super::*;

	/// Initializes the state i.e. instance of a given program
	/// 
	/// # Arguments
	/// 
	/// * `ctx` - The Anchor context holding the accounts
	pub fn initialize(
		ctx: Context<Initialize>,
		protocol_fee: u16,
	) -> Result<()> {
    processors::initialize::exec(ctx, protocol_fee)
	}
}
