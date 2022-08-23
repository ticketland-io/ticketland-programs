#![cfg(feature = "test-bpf")]
use test_context::{test_context, futures};
use solana_sdk::{
  signature::{Signer},
  native_token::sol_to_lamports,
};
use solana_program_test::{tokio};
use solana_test_utils::{
  spl::Spl,
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

// #[test_context(TestContext)]
// #[tokio::test(flavor = "multi_thread")]
// async fn should_enforce_access_control(ctx: &mut TestContext) {
  
// }

// #[test_context(TestContext)]
// #[tokio::test(flavor = "multi_thread")]
// async fn should_transfer_funds(ctx: &mut TestContext) {
  
// }

// #[test_context(TestContext)]
// #[tokio::test(flavor = "multi_thread")]
// async fn should_change_ownership_of_the_ticket(ctx: &mut TestContext) {
  
// }

// #[test_context(TestContext)]
// #[tokio::test(flavor = "multi_thread")]
// async fn should_close_sell_listing_account(ctx: &mut TestContext) {
  
// }

// #[test_context(TestContext)]
// #[tokio::test(flavor = "multi_thread")]
// async fn should_allow_new_owner_list_ticket_for_sale(ctx: &mut TestContext) {
  
// }
