#![cfg(feature = "test-bpf")]
use std::{assert_eq, print};
use test_context::{test_context, futures};
use solana_sdk::{
  signature::{Signer, Keypair},
};
use solana_program_test::{tokio};
use common_test::{
  test_context::TestContext,
  program_id::secondary_market_program_id,
};

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_initialize_secondary_market(ctx: &mut TestContext) {
  let event_registry_state = Keypair::new();
  let secondary_market_state = Keypair::new();
  let ticket_sale_state = Keypair::new();
  let event_registry_runner = &mut ctx.event_registry_runner;
  let ticket_sale_runner = &mut ctx.ticket_sale_runner;
  let secondary_market_runner = &mut ctx.secondary_market_runner;

  event_registry_runner.initialize(
    &event_registry_state,
		1_000, // 10%
  ).await;

  ticket_sale_runner.initialize(
    &ticket_sale_state,
    event_registry_state.pubkey(),
  ).await;

  secondary_market_runner.initialize(
    &secondary_market_state,
    ticket_sale_state.pubkey(),
    500, // 5%
  ).await;

  let mut pt = secondary_market_runner.pt.lock().await;
  let state_data = pt.get_account::<secondary_market::account_data::state::State>(secondary_market_state.pubkey()).await;
  
  assert_eq!(state_data.protocol_fee, 500);
  assert_eq!(state_data.deployer, secondary_market_runner.deployer.pubkey());
  assert_eq!(state_data.ticket_sale_state, ticket_sale_state.pubkey());
  assert_eq!(state_data.ticket_sale_program, secondary_market_program_id());
}
