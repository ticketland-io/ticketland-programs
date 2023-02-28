#![cfg(feature = "test-bpf")]
use test_context::{test_context};
use solana_sdk::{
  signature::{Signer, Keypair},
  pubkey::Pubkey,
  native_token::sol_to_lamports,
};
use solana_program_test::{tokio};
use common_test::{
  test_context::TestContext,
  secondary_market::{
    common::{init, setup},
    error::Error,
    pda
  }
};

async fn before_each(ctx: &mut TestContext) -> (
  Keypair,
  Keypair,
  [u8; 32],
  Pubkey,
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
  let deposit_token = event_registry_runner.deposit_tokens[2];
  let purchase_token = deposit_token;
  let ticket_type_index = 0;

  let (_,) = setup(
    ctx,
    &event_organizer,
    &ticket_buyer,
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
  
  (
    event_registry_state,
    secondary_market_state,
    event_id,
    purchase_token,
  )
}

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_enforce_access_control(ctx: &mut TestContext) {
  let (
    event_registry_state,
    secondary_market_state,
    event_id,
    _,
  ) = before_each(ctx).await;

  let n_listing = 0;
  let event_registry_runner = &mut ctx.event_registry_runner;
  let secondary_market_runner = &mut ctx.secondary_market_runner;
  let ticket_buyer = event_registry_runner.get_participant(3);
  let wrong_purchase_token = event_registry_runner.deposit_tokens[1];

  // should fail is wrong purchase token is provided
  let result = secondary_market_runner.create_buy_listing(
    event_id,
    sol_to_lamports(1.1),
    secondary_market_state.pubkey(),
    event_registry_state.pubkey(),
    wrong_purchase_token,
    &ticket_buyer,
    n_listing,
  ).await;

  Error::assert_err(result, secondary_market::utils::program_error::ErrorCode::WrongPurchaseToken);
}

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_transfer_funds_to_listing_escrow(ctx: &mut TestContext) {
  let (
    event_registry_state,
    secondary_market_state,
    event_id,
    purchase_token,
  ) = before_each(ctx).await;

  let n_listing = 0;
  let event_registry_runner = &mut ctx.event_registry_runner;
  let secondary_market_runner = &mut ctx.secondary_market_runner;
  let ticket_buyer = event_registry_runner.get_participant(3);

  // should fail is wrong purchase token is provided
  let result = secondary_market_runner.create_buy_listing(
    event_id,
    sol_to_lamports(1.1),
    secondary_market_state.pubkey(),
    event_registry_state.pubkey(),
    purchase_token,
    &ticket_buyer,
    n_listing,
  ).await;
  assert!(result.is_ok());

  let escrow_balance = secondary_market_runner.get_listing_escrow_balance(
    secondary_market_state.pubkey(),
    ticket_buyer.pubkey(),
    event_id,
    n_listing,
    purchase_token,
  ).await;

  assert_eq!(escrow_balance, sol_to_lamports(1.1));
}

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_create_buy_listing(ctx: &mut TestContext) {
  let (
    event_registry_state,
    secondary_market_state,
    event_id,
    purchase_token,
  ) = before_each(ctx).await;

  let n_listing = 0;
  let event_registry_runner = &mut ctx.event_registry_runner;
  let secondary_market_runner = &mut ctx.secondary_market_runner;
  let ticket_buyer = event_registry_runner.get_participant(3);

  // should fail is wrong purchase token is provided
  let result = secondary_market_runner.create_buy_listing(
    event_id,
    sol_to_lamports(1.1),
    secondary_market_state.pubkey(),
    event_registry_state.pubkey(),
    purchase_token,
    &ticket_buyer,
    n_listing,
  ).await;
  assert!(result.is_ok());

  {
    let buy_listing = pda::buy_listing(&secondary_market_state.pubkey(), event_id, &ticket_buyer.pubkey(), n_listing).0;
    let (_, listing_escrow_bump) = pda::listing_escrow(&secondary_market_state.pubkey(), event_id, &buy_listing);
    
    let mut pt = secondary_market_runner.pt.lock().await;
    let buy_listing_data = pt.get_account::<secondary_market::account_data::buy_listing::BuyListing>(buy_listing).await;

    assert_eq!(buy_listing_data.bumps.listing_escrow, listing_escrow_bump);
    assert_eq!(buy_listing_data.buyer, ticket_buyer.pubkey());
    assert_eq!(buy_listing_data.bid_price, sol_to_lamports(1.1));
  }
}

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_create_buyer_data(ctx: &mut TestContext) {
  let (
    event_registry_state,
    secondary_market_state,
    event_id,
    purchase_token,
  ) = before_each(ctx).await;

  let n_listing = 0;
  let event_registry_runner = &mut ctx.event_registry_runner;
  let secondary_market_runner = &mut ctx.secondary_market_runner;
  let ticket_buyer = event_registry_runner.get_participant(3);

  // should fail is wrong purchase token is provided
  let result = secondary_market_runner.create_buy_listing(
    event_id,
    sol_to_lamports(1.1),
    secondary_market_state.pubkey(),
    event_registry_state.pubkey(),
    purchase_token,
    &ticket_buyer,
    n_listing,
  ).await;
  assert!(result.is_ok());

  {
    let buyer_data = pda::buyer_data(&secondary_market_state.pubkey(), event_id, &ticket_buyer.pubkey()).0;
    
    let mut pt = secondary_market_runner.pt.lock().await;
    let buyer_data_account_data = pt.get_account::<secondary_market::account_data::buyer_data::BuyerData>(buyer_data).await;

    assert_eq!(buyer_data_account_data.n_listing, 1);
  }
}
