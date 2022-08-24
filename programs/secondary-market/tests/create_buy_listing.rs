#![cfg(feature = "test-bpf")]
use secondary_market::acl::purchase_token;
use test_context::{test_context, futures};
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
    0,
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

  let event_registry_runner = &mut ctx.event_registry_runner;
  let secondary_market_runner = &mut ctx.secondary_market_runner;
  let ticket_buyer = event_registry_runner.get_participant(3);
  let escrow_balance_before = secondary_market_runner.get_listing_escrow_balance();

  // should fail is wrong purchase token is provided
  let result = secondary_market_runner.create_buy_listing(
    event_id,
    sol_to_lamports(1.1),
    secondary_market_state.pubkey(),
    event_registry_state.pubkey(),
    purchase_token,
    &ticket_buyer,
    0,
  ).await;
}
