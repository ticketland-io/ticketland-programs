use anchor_lang::prelude::*;
use std::mem::size_of;
use crate::{
  account_data::{
    state::*,
    sell_listing_reservation::*,
  },
  utils::program_error::ErrorCode,
};

#[derive(Accounts)]
#[instruction(sell_listing: Pubkey)]
pub struct ReserveSellListing<'info> {
  #[account(mut)]
  pub state: Account<'info, State>,

  #[account(
    init,
    payer = operator,
    space = 8 + size_of::<SellListingReservation>(),
    seeds = [
      b"sell_listing_reservation",
      sell_listing.as_ref(),
    ],
    bump,
  )]
  pub sell_listing_reservation: Account<'info, SellListingReservation>,

  #[account(
    mut,
    constraint = state.operators.iter().any(|m| *m == operator.key()) @ ErrorCode::OnlyMintOperator,
  )]
  pub operator: Signer<'info>,

  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
}
