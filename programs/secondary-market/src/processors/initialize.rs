use anchor_lang::prelude::*;
use crate::{
  context::initialize::Initialize,
  account_data::{
    state::*,
  },
};

pub fn exec(
  ctx: Context<Initialize>,
  event_registry_state: Pubkey,
  event_registry_program: Pubkey,
  ticket_sale_state: Pubkey,
  ticket_sale_program: Pubkey,
  ticket_nft_state: Pubkey,
  ticket_nft_program: Pubkey,
  protocol_fee: u16,
) -> Result<()> {
  let state = &mut ctx.accounts.state;

  state.bumps = InitBumps {
    cpi_authority: *ctx.bumps.get("cpi_authority").unwrap(),
  };
  state.event_registry_state = event_registry_state;
  state.event_registry_program = event_registry_program;
  state.ticket_sale_state = ticket_sale_state;
  state.ticket_sale_program = ticket_sale_program;
  state.ticket_nft_state = ticket_nft_state;
  state.ticket_nft_program = ticket_nft_program;
  state.deployer = ctx.accounts.deployer.key();
  state.protocol_fee = protocol_fee;

  Ok(())
}
