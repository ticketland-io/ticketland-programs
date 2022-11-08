use anchor_lang::prelude::*;
use crate::{
  account_data::{
    state::*,
    sell_listing_reservation::*,
  },
  utils::program_error::ErrorCode,
};

#[derive(Accounts)]
#[instruction(sell_listing: Pubkey)]
pub struct CloseSellListingReservation<'info> {
  #[account()]
  pub state: Account<'info, State>,

  #[account(
    mut,
    close = operator,
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
}
