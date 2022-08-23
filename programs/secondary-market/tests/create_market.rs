#![cfg(feature = "test-bpf")]
use test_context::{test_context, futures};
use solana_sdk::{
  signature::{Signer},
};
use solana_program_test::{tokio};
use common_test::{
  test_context::TestContext,
  secondary_market::{
    common::{init, setup},
    pda,
    error::Error,
  }
};

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_create_new_market(ctx: &mut TestContext) {
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

  let _ = setup(
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
  ).await;

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

  {
    let (market, market_bump) = pda::market(&secondary_market_state.pubkey(), event_id);
    let mut pt = secondary_market_runner.pt.lock().await;
    let market_data = pt.get_account::<secondary_market::account_data::market::Market>(market).await;
    
    assert_eq!(market_data.event_id, event_id);
    assert_eq!(market_data.bumps.market, market_bump);
    assert_eq!(market_data.organizer_resale_fee, 500);
    assert_eq!(market_data.resale_cap, 1000);
  }
  
  // should update the secondary market state as well
  {
    let mut pt = secondary_market_runner.pt.lock().await;
    let state_data = pt.get_account::<secondary_market::account_data::state::State>(secondary_market_state.pubkey()).await;
    assert_eq!(state_data.n_markets, 1);
  }
}

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_only_be_called_by_the_event_organizer(ctx: &mut TestContext) {
  let (
    event_registry_state,
    ticket_sale_state,
    ticket_nft_state,
    secondary_market_state,
  ) = init(ctx).await;

  let event_id: [u8; 32] = "85ac6394e04a4b3c8ccd7e2772cb14b4".to_owned().into_bytes().try_into().unwrap();
  let seat_index = 0;

  {
    let event_registry_runner = &mut ctx.event_registry_runner;
    let event_organizer = event_registry_runner.get_participant(1);
    let ticket_buyer = event_registry_runner.get_participant(2);
    let deposit_token = event_registry_runner.deposit_tokens[2];
    let purchase_token = deposit_token;

    let _ = setup(
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
    ).await;
  }

  {
    let event_registry_runner = &mut ctx.event_registry_runner;
    let wrong_event_organizer = event_registry_runner.get_participant(3);
    let secondary_market_runner = &mut ctx.secondary_market_runner;
    let result = secondary_market_runner.create_market(
      secondary_market_state.pubkey(),
      event_registry_state.pubkey(),
      event_id,
      &wrong_event_organizer,
      500, // organizer_resale_fee 5%
      1000, // resale_cap 10%
    ).await;

    Error::assert_err(result, secondary_market::utils::program_error::ErrorCode::OnlyEventOrganizer);
  }
}
