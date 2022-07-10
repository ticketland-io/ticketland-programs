use anchor_lang::prelude::*;
use crate::{
  context::purchase::*,
};

pub fn exec(
  ctx: Context<Purchase>,
  seat_index: u32,
  seat_name: String,
) -> Result<()> {
  Ok(())
}
