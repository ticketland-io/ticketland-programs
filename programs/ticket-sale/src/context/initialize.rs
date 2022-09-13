use anchor_lang::prelude::*;
use std::mem::size_of;
use crate::{
  account_data::{
    state::*,
  },
};

#[derive(Accounts)]
pub struct Initialize<'info> {
  // The state account of each instance of this program
  #[account(
    init,
    payer = deployer,
    space = 8 + size_of::<State>()
  )]
  pub state: Account<'info, State>,

  /// CHECK: THe PDA that will be sending CPI to other programs i.e. TicketSale Program
  #[account(
    init,
    payer = deployer,
    space = 0,
    seeds = [b"ticket_sale:cpi_authority", state.key().as_ref()],
    bump,
  )]
  pub cpi_authority: AccountInfo<'info>,
  
  #[account(mut)]
  pub deployer: Signer<'info>,
  
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
}
