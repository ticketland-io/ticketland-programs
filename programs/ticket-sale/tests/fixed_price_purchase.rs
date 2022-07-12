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
  event_registry::{
    runner::Runner as EventRegistryRunner,
  },
  program_id::event_registry_program_id,
};

async fn init(ctx: &mut TestContext) -> (Keypair, Keypair, Keypair) {
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

  (
    event_registry_state,
    ticket_sale_state,
    ticket_nft_state,
  )
}

async fn create_event(
  runner: &mut EventRegistryRunner,
  event_registry_state: &Keypair,
  event_capacity: Pubkey,
  event_id: u64,
  event_organizer: &Keypair,
  deposit_token: Pubkey,
  ticket_types: Vec<TicketType>
) {
  let _ = runner.create_event(
    state.pubkey(),
    event_capacity,
    ticket_sale_program_state,
    event_id,
    deposit_token,
    deposit_token,
    &event_organizer,
    event_organizer.pubkey(),
    10, // num of tickets
    100,
		1000,
		ticket_types.clone(),
		"Ticket Land Coolest Event".to_owned(),
		"TICKT".to_owned(),
		"https://ticketland.io".to_owned(),
  ).await;
}

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_allow_ticket_buyer_to_purchase_ticket_on_fixed_price_using_sol(ctx: &mut TestContext) {
  let (
    event_registry_state,
    ticket_sale_state,
    ticket_nft_state,
  ) = init(ctx).await;

  let event_registry_runner = &mut ctx.event_registry_runner;
  let ticket_sale_runner = &mut ctx.ticket_sale_runner;
  let event_capacity = event_registry_runner.create_event_capacity_account().await;
  let event_id = 0;
  let event_organizer = runner.get_participant(1);
  let deposit_token = runner.deposit_tokens[0];

  let ticket_types = vec![
    TicketType {
      n_tickets: 50_000,
      sale_type: SaleType::FixedPrice(to_base(100, 6)),
      sale_start_time: 50,
      merkle_root: [0; 32],
    },
    TicketType {
      n_tickets: 50_000,
      sale_type: SaleType::DutchAuction {
        start_price: 150,
        end_price: 110,
        curve_length: 200 * 60,
        drop_interval: 20 * 60,
      },
      sale_start_time: 50,
      merkle_root: [0; 32],
    },
  ];

  
  custom_create_event(
    event_registry_runner,
    ticket_sale_runner,
    &event_registry_state,
    event_capacity,
    event_id,
    &event_organizer,
    deposit_token,
  ).await;
}
