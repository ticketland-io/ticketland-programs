#![cfg(feature = "test-bpf")]

use std::{assert_eq};
use test_context::{test_context, futures};
use solana_sdk::{
  signature::{Signer, Keypair},
};
use solana_program_test::{tokio};
use common_test::{
  ticket_sale::pda,
  test_context::TestContext,
};

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_initialize_ticket_sale(ctx: &mut TestContext) {
  let state = Keypair::new();
  let event_registry_runner = &mut ctx.event_registry_runner;
  let ticket_sale_runner = &mut ctx.ticket_sale_runner;
  
  event_registry_runner.initialize(
    &state,
    500, // 5%
		1_000, // 10%
  ).await;
}
