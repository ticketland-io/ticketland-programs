#![cfg(feature = "test-bpf")]
use std::{assert_eq};
use test_context::{test_context, futures};
use solana_sdk::{
  signature::{Signer, Keypair},
};
use solana_program_test::{tokio};
use common_test::{
  test_context::TestContext,
  ticket_sale::pda,
  program_id::event_registry_program_id,
};

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_initialize_ticket_sale(ctx: &mut TestContext) {
  let event_registry_state = Keypair::new();
  let ticket_sale_state = Keypair::new();
  let event_registry_runner = &mut ctx.event_registry_runner;
  let ticket_sale_runner = &mut ctx.ticket_sale_runner;
  
  event_registry_runner.initialize(
    &event_registry_state,
    500, // 5%
		1_000, // 10%
  ).await;

  ticket_sale_runner.initialize(
    &ticket_sale_state,
    event_registry_state.pubkey(),
  ).await;

  let mut pt = ticket_sale_runner.pt.lock().await;
  let state_data = pt.get_account::<ticket_sale::account_data::state::State>(ticket_sale_state.pubkey()).await;

  let (cpi_authority, cpi_authority_bump) = pda::cpi_authority(&ticket_sale_state.pubkey());
  
  assert_eq!(state_data.bumps.cpi_authority, cpi_authority_bump);
  assert_eq!(state_data.cpi_authority, cpi_authority);
  assert_eq!(state_data.treasury, ticket_sale_runner.treasury.pubkey());
  assert_eq!(state_data.event_registry_program, event_registry_program_id());
  assert_eq!(state_data.event_registry_state, event_registry_state.pubkey());
  assert_eq!(state_data.deployer, ticket_sale_runner.deployer.pubkey());
}
