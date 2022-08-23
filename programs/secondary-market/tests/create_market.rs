#![cfg(feature = "test-bpf")]
use anchor_lang::{
  prelude::{
    Pubkey,
  },
};
use test_context::{test_context, futures};
use solana_sdk::{
  signature::{Signer, Keypair},
  native_token::sol_to_lamports,
};
use solana_program_test::{tokio};
use common::{
  state::{
    ticket_type::{TicketType, SeatRange},
    sale_type::SaleType,
  },
};
use common_test::{
  test_context::TestContext,
  ticket_sale::{
    runner::Runner as TicketSaleRunner,
  },
  secondary_market::{
    common::{init, setup},
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

  let result = secondary_market_runner.create_market(
    event_id,
    sol_to_lamports(1.05), // 5% higher than the price sold in the primary market
    secondary_market_state,
    sale: Pubkey,
    seat_index: u32,
    event: Pubkey,
    ticket_nft_state,
    purchase_token,
    &ticket_buyer,
  ).await;

  assert!(result.is_ok());
}
