#![cfg(feature = "test-bpf")]
use test_context::{test_context, futures};
use solana_sdk::{
  signature::{Signer, Keypair},
  pubkey::Pubkey,
  native_token::sol_to_lamports,
};
use solana_program_test::{tokio};
use common::{
  state::{
    ticket_type::{TicketType},
  },
};
use common_test::{
  test_context::TestContext,
  ticket_sale::{
    pda::{self as TicketSalePda},
  },
  ticket_nft::{
    pda as TicketNftPda,
  },
  secondary_market::{
    common::{init, setup},
    error::Error,
    pda
  }
};

async fn before_each(ctx: &mut TestContext, bid_price: u64) -> (
  Keypair,
  Keypair,
  Keypair,
  Keypair,
  Keypair,
  Keypair,
  Keypair,
  Pubkey,
  [u8; 32],
  u8,
  u16,
  u32,
  Vec<TicketType>
 ) {
  let (
    event_registry_state,
    ticket_sale_state,
    ticket_nft_state,
    secondary_market_state,
  ) = init(ctx).await;

  let event_id: [u8; 32] = "85ac6394e04a4b3c8ccd7e2772cb14b4".to_owned().into_bytes().try_into().unwrap();
  let seat_index = 0;
  let event_registry_runner = &mut ctx.event_registry_runner;
  let event_organizer = event_registry_runner.get_participant(1);
  let ticket_buyer = event_registry_runner.get_participant(2);
  let ticket_owner = event_registry_runner.get_participant(3);
  let deposit_token = event_registry_runner.deposit_tokens[2];
  let purchase_token = deposit_token;
  let ticket_type_index = 0;
  let n_listing = 0;

  let (ticket_types,) = setup(
    ctx,
    &event_organizer,
    &ticket_owner,
    event_registry_state.pubkey(),
    ticket_sale_state.pubkey(),
    ticket_nft_state.pubkey(),
    deposit_token,
    purchase_token,
    event_id,
    seat_index,
    ticket_type_index,
  ).await;

  {
    let secondary_market_runner = &mut ctx.secondary_market_runner;
    
    let result = secondary_market_runner.create_market(
      secondary_market_state.pubkey(),
      event_registry_state.pubkey(),
      event_id,
      &event_organizer,
      500, // organizer_resale_fee 5%
      1000, // resale_cap 10%
    ).await;
    assert!(result.is_ok());
  }
  
  {
    let secondary_market_runner = &mut ctx.secondary_market_runner;
    let result = secondary_market_runner.create_buy_listing(
      event_id,
      bid_price,
      secondary_market_state.pubkey(),
      event_registry_state.pubkey(),
      purchase_token,
      &ticket_buyer,
      n_listing,
    ).await;
    assert!(result.is_ok());
  }

  (
    event_registry_state,
    secondary_market_state,
    ticket_sale_state,
    ticket_nft_state,
    ticket_buyer,
    ticket_owner,
    event_organizer,
    purchase_token,
    event_id,
    ticket_type_index,
    n_listing,
    seat_index,
    ticket_types,
  )
}

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_fail_if_sale_ended(ctx: &mut TestContext) {
  let (
    event_registry_state,
    secondary_market_state,
    ticket_sale_state,
    ticket_nft_state,
    ticket_buyer,
    ticket_owner,
    event_organizer,
    purchase_token,
    event_id,
    ticket_type_index,
    n_listing,
    seat_index,
    ticket_types,
  ) = before_each(ctx, sol_to_lamports(1.1)).await;

  let sale = TicketSalePda::ticket_sale_state(
    &ticket_sale_state.pubkey(),
    ticket_type_index,
    event_id,
  ).0;

  // move to the end of sale
  {
    let ticket_sale_runner = &mut ctx.ticket_sale_runner;
    let mut pt = ticket_sale_runner.pt.lock().await;
    pt.advance_clock_past_timestamp(ticket_types[0].sale_end_time + 1).await;
  }

  let secondary_market_runner = &mut ctx.secondary_market_runner;
  let event_registry_runner = &mut ctx.event_registry_runner;
  let treasury = event_registry_runner.get_participant(5);

  let result = secondary_market_runner.fill_buy_listing(
    event_id,
    secondary_market_state.pubkey(),
    event_registry_state.pubkey(),
    ticket_nft_state.pubkey(),
    sale,
    purchase_token,
    treasury.pubkey(),
    ticket_buyer.pubkey(),
    event_organizer.pubkey(),
    &ticket_owner,
    n_listing,
    seat_index,
  ).await;
  
  Error::assert_ticket_sale_err(result, ticket_sale::utils::program_error::ErrorCode::SaleFinished);
}

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_fail_if_sale_account_is_wrong(ctx: &mut TestContext) {
  let (
    event_registry_state,
    secondary_market_state,
    ticket_sale_state,
    ticket_nft_state,
    ticket_buyer,
    ticket_owner,
    event_organizer,
    purchase_token,
    event_id,
    ticket_type_index,
    n_listing,
    seat_index,
    _,
  ) = before_each(ctx, sol_to_lamports(1.1)).await;

  let some_other_event_id: [u8; 32] = "aaaa6394e04a4b3c8ccd7e2772cb14b4".to_owned().into_bytes().try_into().unwrap();
  
  {
    // prepare new accounts
    let event_registry_runner = &mut ctx.event_registry_runner;
    let deposit_token = event_registry_runner.deposit_tokens[2];
    // create a new event
    let _ = setup(
      ctx,
      &event_organizer,
      &ticket_owner,
      event_registry_state.pubkey(),
      ticket_sale_state.pubkey(),
      ticket_nft_state.pubkey(),
      deposit_token,
      purchase_token,
      some_other_event_id,
      seat_index,
      ticket_type_index,
    ).await;

    let secondary_market_runner = &mut ctx.secondary_market_runner;

    // create a market for that sale
    let result = secondary_market_runner.create_market(
      secondary_market_state.pubkey(),
      event_registry_state.pubkey(),
      some_other_event_id,
      &event_organizer,
      500, // organizer_resale_fee 5%
      1000, // resale_cap 10%
    ).await;
    assert!(result.is_ok());
  }

  // now use this new sale to create listing for a ticket that was purchased in the previous sale
  let sale = TicketSalePda::ticket_sale_state(
    &ticket_sale_state.pubkey(),
    ticket_type_index,
    some_other_event_id,
  ).0;

  let secondary_market_runner = &mut ctx.secondary_market_runner;
  let event_registry_runner = &mut ctx.event_registry_runner;
  let treasury = event_registry_runner.get_participant(5);

  let result = secondary_market_runner.fill_buy_listing(
    event_id,
    secondary_market_state.pubkey(),
    event_registry_state.pubkey(),
    ticket_nft_state.pubkey(),
    sale,
    purchase_token,
    treasury.pubkey(),
    ticket_buyer.pubkey(),
    event_organizer.pubkey(),
    &ticket_owner,
    n_listing,
    seat_index,
  ).await;

  Error::assert_err(result, secondary_market::utils::program_error::ErrorCode::WrongSaleAccount);
}

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_fail_if_not_ticket_metadata_owner(ctx: &mut TestContext) {
  let (
    event_registry_state,
    secondary_market_state,
    ticket_sale_state,
    ticket_nft_state,
    ticket_buyer,
    _,
    event_organizer,
    purchase_token,
    event_id,
    ticket_type_index,
    n_listing,
    seat_index,
    _,
  ) = before_each(ctx, sol_to_lamports(1.1)).await;

  let sale = TicketSalePda::ticket_sale_state(
    &ticket_sale_state.pubkey(),
    ticket_type_index,
    event_id,
  ).0;

  let secondary_market_runner = &mut ctx.secondary_market_runner;
  let event_registry_runner = &mut ctx.event_registry_runner;
  let treasury = event_registry_runner.get_participant(5);
  let wrong_ticket_owner = event_registry_runner.get_participant(6);

  let result = secondary_market_runner.fill_buy_listing(
    event_id,
    secondary_market_state.pubkey(),
    event_registry_state.pubkey(),
    ticket_nft_state.pubkey(),
    sale,
    purchase_token,
    treasury.pubkey(),
    ticket_buyer.pubkey(),
    event_organizer.pubkey(),
    &wrong_ticket_owner,
    n_listing,
    seat_index,
  ).await;
  
  Error::assert_err(result, secondary_market::utils::program_error::ErrorCode::OnlyTicketOwner);
}

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_fail_if_price_cap_exceed(ctx: &mut TestContext) {
  let (
    event_registry_state,
    secondary_market_state,
    ticket_sale_state,
    ticket_nft_state,
    ticket_buyer,
    ticket_owner,
    event_organizer,
    purchase_token,
    event_id,
    ticket_type_index,
    n_listing,
    seat_index,
    _,
  ) = before_each(ctx, sol_to_lamports(1.11)).await;

  let sale = TicketSalePda::ticket_sale_state(
    &ticket_sale_state.pubkey(),
    ticket_type_index,
    event_id,
  ).0;

  let secondary_market_runner = &mut ctx.secondary_market_runner;
  let event_registry_runner = &mut ctx.event_registry_runner;
  let treasury = event_registry_runner.get_participant(5);

  let result = secondary_market_runner.fill_buy_listing(
    event_id,
    secondary_market_state.pubkey(),
    event_registry_state.pubkey(),
    ticket_nft_state.pubkey(),
    sale,
    purchase_token,
    treasury.pubkey(),
    ticket_buyer.pubkey(),
    event_organizer.pubkey(),
    &ticket_owner,
    n_listing,
    seat_index,
  ).await;
  
  // The buy listing has an ask price of 1.11 but the ticket was initially sold for 1 SOL
  // This exceeds the price cap thus it should fail
  Error::assert_err(result, secondary_market::utils::program_error::ErrorCode::PriceCap);
}
