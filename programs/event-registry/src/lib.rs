pub mod context;
pub mod account_data;
pub mod processors;
pub mod acl;
pub mod utils;

use anchor_lang::prelude::*;
use crate::{
	context::{
		initialize::*,
	},
};

declare_id!("TGfdMZj2HoSwdFR5zUAKr8H72XYJ85GQ7my5yZTHGKE");

#[program]
pub mod ticker_land_programs {
use super::*;
	pub fn initialize(
		ctx: Context<Initialize>,
		supported_currencies: Vec<Pubkey>,
	) -> Result<()> {
    processors::initialize::exec(ctx, supported_currencies)
	}
}
