use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};
use common::{
  account_data::{
    serialization::deser,
  },
  state::{
    sale_type::*,
  },
};
use crate::{
  context::fixed_price_purchase::*,
  account_data::{
    event::Event,
  },
  utils::program_error::ErrorCode,
};
use super::common_purchase::{
  fixed_price_purchase_pre_checks,
  post_checks,
};

fn transfer_token<'info>(
  ctx: &Context<FixedPricePurchase<'info>>,
  from: AccountInfo<'info>,
  to: AccountInfo<'info>,
  authority: AccountInfo<'info>,
  amount: u64,
) -> Result<()> {
  let cpi_accounts = Transfer::<'info> {from, to, authority};
  let cpi_program = ctx.accounts.token_program.to_account_info();
  let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

  token::transfer(cpi_ctx, amount)
}

/// Transfer the purchase funds to event organizer and our treasury
fn transfer_funds(ctx: &Context<FixedPricePurchase>, event: &Event) -> Result<u64> {
  let ticket_type = &ctx.accounts.sale.ticket_type;
  let amount = if let SaleType::FixedPrice {amount} = ticket_type.sale_type {
    amount
  } else {
    // This should never happen since we already have the same check in the FixedPricePurchase context
    return Err(ErrorCode::UnexpectedSaleAccount.into());
  };
  let (event_organizer_amount, service_fee_amount) = event.currency.calc_fee(amount)?;

  // send to treasury
  if service_fee_amount > 0 {
    transfer_token(
      &ctx,
      ctx.accounts.ticket_buyer_ata.to_account_info().clone(),
      ctx.accounts.service_fee_ata.to_account_info().clone(),
      ctx.accounts.ticket_buyer.to_account_info().clone(),
      service_fee_amount,
    )?;
  }

  // send to event organizer
  if event_organizer_amount > 0 {
    transfer_token(
      &ctx,
      ctx.accounts.ticket_buyer_ata.to_account_info().clone(),
      ctx.accounts.event_organizer_purchase_token_ata.to_account_info().clone(),
      ctx.accounts.ticket_buyer.to_account_info().clone(),
      event_organizer_amount,
    )?;
  }

  Ok(amount)
}

fn mint_ticket(ctx: &Context<FixedPricePurchase>, price_sold: u64, seat_index: u32, seat_name: String) -> Result<()> {
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
    price_sold,
		seat_name,
  )?;

  Ok(())
}

pub fn exec(
  ctx: Context<FixedPricePurchase>,
  seat_index: u32,
  seat_name: String,
) -> Result<()> {
  let event: Event = deser(ctx.accounts.event.clone())?;

  fixed_price_purchase_pre_checks(&ctx, &event, seat_index)?;
  let price_sold = transfer_funds(&ctx, &event)?;
  mint_ticket(&ctx, price_sold, seat_index, seat_name)?;
  post_checks(&mut ctx.accounts.state, &mut ctx.accounts.event_capacity, seat_index)?;

  Ok(())
}
