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


declare_id!("599YwRjALAKVj7z9bcBijrYHyNGLTJSjmJTzeyttnEFL");

#[program]
pub mod ticket_nft {
	use super::*;

	/// Initializes the state i.e. instance of a given program
	/// 
	/// # Arguments
	/// 
	/// * `ctx` - The Anchor context holding the accounts
	pub fn initialize(
		ctx: Context<Initialize>,
	) -> Result<()> {
    processors::initialize::exec(ctx)
	}
}
