#![cfg(feature = "test-bpf")]
use anchor_lang::{prelude::{Pubkey}};
use test_context::{test_context, futures};
use solana_sdk::{
  signature::{Signer, Keypair},
  native_token::sol_to_lamports,
};
use solana_program_test::{tokio};
use common::{
  state::{
    ticket_type::TicketType,
    sale_type::SaleType,
  },
};
// use solana_test_utils::{
//   utils::{to_base},
// };
use common_test::{
  test_context::TestContext,
  event_registry::{
    runner::Runner as EventRegistryRunner,
  },
  ticket_sale::{
    runner::Runner as TicketSaleRunner,
  },
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

  ticket_sale_runner.create_treasury_atas(&event_registry_runner.deposit_tokens.clone()).await;

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

async fn custom_create_event(
  event_registry_runner: &mut EventRegistryRunner,
  event_registry_state: Pubkey,
  ticket_sale_program_state: Pubkey,
  event_capacity: Pubkey,
  event_id: u64,
  event_organizer: &Keypair,
  event_organizer_treasury: Pubkey,
  deposit_token: Pubkey,
  ticket_types: &Vec<TicketType>
) {
  let _ = event_registry_runner.create_event(
    event_registry_state,
    event_capacity,
    ticket_sale_program_state,
    event_id,
    deposit_token,
    deposit_token,
    &event_organizer,
    event_organizer_treasury,
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
  let event_organizer = event_registry_runner.get_participant(1);
  let deposit_token = event_registry_runner.deposit_tokens[0];

  // ticket type 1 includes seats 0, 1, 2, 5, 6, 7
  let mt_type_1 = ticket_sale_runner.create_ticket_type_mt(vec![(0, 2), (5, 7)]);
  // ticket type 3 includes seats 3, 4, 8, 9
  let mt_type_2 = ticket_sale_runner.create_ticket_type_mt(vec![(3, 4), (8, 9)]);

  let ticket_types = vec![
    TicketType {
      n_tickets: 4,
      sale_type: SaleType::FixedPrice(sol_to_lamports(1_f64)),
      sale_start_time: 50,
      merkle_root: mt_type_1.root().unwrap(),
    },
    TicketType {
      n_tickets: 6,
      sale_type: SaleType::DutchAuction {
        start_price: 150,
        end_price: 110,
        curve_length: 200 * 60,
        drop_interval: 20 * 60,
      },
      sale_start_time: 50,
      merkle_root: mt_type_2.root().unwrap(),
    },
  ];

  custom_create_event(
    event_registry_runner,
    event_registry_state.pubkey(),
    ticket_sale_state.pubkey(),
    event_capacity,
    event_id,
    &event_organizer,
    event_organizer.pubkey(),
    deposit_token,
    &ticket_types,
  ).await;

  // Create a new ticket sale for the first ticket type
  let _ = event_registry_runner.create_ticket_sale(
    event_registry_state.pubkey(),
    event_id,
    &event_organizer,
    ticket_sale_state.pubkey(),
    0, // ticket_type_index
    ticket_types[0].clone(),
  ).await;

  let ticket_buyer = event_registry_runner.get_participant(2);
  let purchase_token = event_registry_runner.deposit_tokens[0];

  let result = ticket_sale_runner.fixed_price_purchase(
    &ticket_buyer,
    event_registry_state.pubkey(),
    ticket_sale_state.pubkey(),
    event_capacity,
    purchase_token,
    event_organizer.pubkey(),
    event_organizer.pubkey(),
    ticket_nft_state.pubkey(),
    event_id,
    0, // ticket_type_index
    0, // seat_index,
		TicketSaleRunner::dummy_seat_name(0),
		mt_type_1.proof(&[0]), // proof path for leaf 0
  ).await;

  assert!(result.is_ok());
}
