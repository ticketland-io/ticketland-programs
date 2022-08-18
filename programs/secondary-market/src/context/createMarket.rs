use anchor_lang::prelude::*;
use std::mem::size_of;
use crate::{
  account_data::{
    state::*,
    market::*,
  },
};

#[derive(Accounts)]
#[instruction(event_id: [u8; 32])]
pub struct CreateMarket<'info> {
  // The state account of each instance of this program
  #[account(mut)]
  pub state: Account<'info, State>,

  // The state account of each instance of this program
  #[account(
    init,
    payer = event_organizer,
    space = 8 + size_of::<Market>() + SPACE_MARGIN
  )]
  pub market: Account<'info, Market>,

  /// CHECK: The Event account.
  /// Constraints will be checked in the processor fn
  #[account(
    seeds = [
      b"event",
      state.event_registry_state.key().as_ref(),
      &event_id
    ],
    bump,
    seeds::program = state.event_registry_program,
  )]
  pub event: AccountInfo<'info>,

  /// This is the user that created the event earlier
  /// event_organizer.key() == event.event_organizer will be checked in the processor
  #[account(mut)]
  pub event_organizer: Signer<'info>,

  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
}
