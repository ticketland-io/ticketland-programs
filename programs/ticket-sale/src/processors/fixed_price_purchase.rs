use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
  program::invoke,
  system_instruction::transfer,
};
use anchor_spl::token::{self, Transfer};
use common::{
  utils::bitmap,
  state::{
    sale_type::*,
  },
  token::is_wrapped_sol,
};
use crate::{
  context::fixed_price_purchase::*,
  account_data::event_capacity::MAX_VENUE_CAPACITY,
  acl::seat_validity,
  utils::program_error::ErrorCode,
};

fn transfer_sol<'info>(
  from: AccountInfo<'info>,
  to: AccountInfo<'info>,
  amount: u64,
) -> Result<()> {
  if amount > 0 {
    let ix = transfer(
      &from.key(),
      &to.key(),
      amount
    );

    return invoke(
      &ix,
      &[from, to],
    ).map_err(|err| err.into())
  }

  Ok(())
}

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
fn transfer_funds(ctx: &Context<FixedPricePurchase>) -> Result<()> {
  let ticket_type = &ctx.accounts.sale.ticket_type;
  let amount = if let SaleType::FixedPrice(amount) = ticket_type.sale_type {
    amount
  } else {
    // This should never happen since we already have the same check in the FixedPricePurchase context
    return Err(ErrorCode::ExpectedFixedPriceSaleAccount.into());
  };
  let (event_organizer_amount, service_fee_amount) = ctx.accounts.event.currency.calc_fee(amount)?;

  if is_wrapped_sol(ctx.accounts.purchase_token.key()) {
    // send to event organizer
    transfer_sol(
      ctx.accounts.ticket_buyer_ata.to_account_info().clone(),
      ctx.accounts.event_organizer_purchase_token_ata.to_account_info().clone(),
      event_organizer_amount,
    )?;

    // send to treasury
    transfer_sol(
      ctx.accounts.ticket_buyer_ata.to_account_info().clone(),
      ctx.accounts.service_fee_ata.to_account_info().clone(),
      service_fee_amount,
    )?;
  } else {
    // transfer_token(
    //   &ctx,
    //   from: ctx.accounts.
    // )?;
  }

  Ok(())
}

// 1. Make sure that the given params belong to the Sale's ticket_type sparse MT
#[access_control(seat_validity::verify(
  ctx.accounts.sale.ticket_type.merkle_root,
  merkle_proof,
  seat_index,
  seat_name,
))]
pub fn exec(
  ctx: Context<FixedPricePurchase>,
  seat_index: u32,
  seat_name: String,
  merkle_proof: Vec<[u8; 32]>,
) -> Result<()> {
  // 2. Has sale started?
  let sale = &ctx.accounts.sale;
  require!(Clock::get().unwrap().slot >= sale.ticket_type.sale_start_time, ErrorCode::SaleNotStarted);

  let event_capacity = &mut ctx.accounts.event_capacity.load_mut()?;
  // 3. Are there any available seats for this type of ticket
  require!(event_capacity.available_tickets > 0, ErrorCode::TicketSoldOut);

  // 4. Check that the seat_index is available
  require!(
    bitmap::is_true::<u8, MAX_VENUE_CAPACITY>(seat_index, &event_capacity.seats),
    ErrorCode::SeatNotAvailable,
  );

  // 5. Transfer funds
  transfer_funds(&ctx)?;

  // 6. CPI to ticket NFT to mint the ticket
  Ok(())
}
