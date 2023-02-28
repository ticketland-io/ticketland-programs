#![cfg(feature = "test-bpf")]
use std::{assert_eq};
use test_context::{test_context};
use solana_sdk::{
  signature::{Signer, Keypair},
};
use solana_program_test::{tokio};
use common_test::{
  ticket_nft::pda,
  test_context::TestContext,
  program_id::{
    ticket_sale_program_id,
  },
};

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_initialize_ticket_nft(ctx: &mut TestContext) {
  let event_registry_state = Keypair::new();
  let ticket_sale_state = Keypair::new();
  let ticket_nft_state = Keypair::new();
  let event_registry_runner = &mut ctx.event_registry_runner;
  let ticket_sale_runner = &mut ctx.ticket_sale_runner;
  let ticket_nft_runner = &mut ctx.ticket_nft_runner;
  
  event_registry_runner.initialize(
    &event_registry_state,
		1_000, // 10%
  ).await;

  ticket_sale_runner.initialize(
    &ticket_sale_state,
    event_registry_state.pubkey(),
  ).await;

  ticket_nft_runner.initialize(
    &ticket_nft_state,
    ticket_sale_state.pubkey(),
  ).await;

  let mut pt = ticket_nft_runner.pt.lock().await;
  let state_data = pt.get_account::<ticket_nft::account_data::state::State>(ticket_nft_state.pubkey()).await;
  let (nft_authority, nft_authority_bump) = pda::nft_authority(&ticket_nft_state.pubkey());

  assert_eq!(state_data.nft_authority, nft_authority);
  assert_eq!(state_data.bumps.nft_authority, nft_authority_bump);
  assert_eq!(state_data.ticket_sale_program, ticket_sale_program_id());
  assert_eq!(state_data.ticket_sale_state, ticket_sale_state.pubkey());
  assert_eq!(state_data.deployer, ticket_nft_runner.deployer.pubkey());
}
