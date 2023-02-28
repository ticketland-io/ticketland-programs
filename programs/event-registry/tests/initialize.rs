#![cfg(feature = "test-bpf")]

use std::{assert_eq};
use test_context::{test_context};
use solana_sdk::{
  signature::{Signer, Keypair},
};
use solana_program_test::{tokio};
use common_test::{
  event_registry::pda,
  test_context::TestContext,
};

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_initialize(ctx: &mut TestContext) {
  let state = Keypair::new();
  let runner = &mut ctx.event_registry_runner;
  
  runner.initialize(
    &state,
		1_000, // 10%
  ).await;

  let mut pt = runner.pt.lock().await;
  let state_data = pt.get_account::<event_registry::account_data::state::State>(state.pubkey()).await;
  let (cpi_authority, cpi_authority_bump) = pda::cpi_authority(&state.pubkey());

  assert_eq!(state_data.deployer, runner.deployer.pubkey());
  assert_eq!(state_data.bumps.event_nft_authority, pda::event_nft_authority(&state.pubkey()).1);
  assert_eq!(state_data.bumps.cpi_authority, cpi_authority_bump);
  assert_eq!(state_data.seller_fee_basis_points, 1_000);
  assert_eq!(state_data.supported_currencies, runner.supported_currencies);
  assert_eq!(state_data.cpi_authority, cpi_authority);
}
