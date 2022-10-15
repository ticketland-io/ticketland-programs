use anchor_lang::prelude::*;
use common::{
  account_data::{
    serialization::deser,
  },
};
use crate::{
  account_data::{
    event::Event,
  },
  context::free_purchase::FreePurchase,
};
use super::common_purchase::{
  free_purchase_pre_checks,
  post_checks,
};

fn mint_ticket(ctx: &Context<FreePurchase>, seat_index:  u32, seat_name: String) -> Result<()> {
  let cpi_program = ctx.accounts.ticket_nft_program.to_account_info();
  let cpi_accounts = ticket_nft::cpi::accounts::CreateTicket {
    state: ctx.accounts.ticket_nft_program_state.to_account_info(),
    ticket_metadata: ctx.accounts.ticket_metadata.to_account_info(),
    nft_authority: ctx.accounts.nft_authority.to_account_info(),
    nft: ctx.accounts.ticket_nft.to_account_info(),
    event_nft_metadata: ctx.accounts.event_nft_metadata.to_account_info(),
    ticket_nft_ata: ctx.accounts.ticket_nft_ata.to_account_info(),
    ticket_sale_cpi_authority: ctx.accounts.cpi_authority.to_account_info(),
    ticket_buyer: ctx.accounts.ticket_buyer.to_account_info(),
    token_program: ctx.accounts.token_program.to_account_info(),
    associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
    system_program: ctx.accounts.system_program.to_account_info(),
    rent: ctx.accounts.rent.to_account_info(),
  };
  
  let state = &ctx.accounts.state;
  let state_key = state.key();
  let seeds: &[&[u8]] = &[
    b"ticket_sale:cpi_authority", state_key.as_ref(),
    &[state.bumps.cpi_authority]
  ];
  let signer_seeds:&[&[&[u8]]] = &[&seeds[..]];

  let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

  ticket_nft::cpi::create_ticket(
    cpi_ctx,
		ctx.accounts.state.bumps.cpi_authority,
    ctx.accounts.sale.ticket_type_index,
		ctx.accounts.sale.event_id,
    seat_index,
    ctx.accounts.ticket_buyer.key(),
    ctx.accounts.sale.key(),
    0_u64,
		seat_name,
  )?;

  Ok(())
}

pub fn exec(
  ctx: Context<FreePurchase>,
  seat_index: u32,
  seat_name: String,
) -> Result<()> {
  let event: Event = deser(ctx.accounts.event.clone())?;

  free_purchase_pre_checks(&ctx, &event, seat_index)?;
  mint_ticket(&ctx, seat_index, seat_name)?;
  post_checks(&mut ctx.accounts.state, &mut ctx.accounts.event_capacity, seat_index)?;

  Ok(())
}
