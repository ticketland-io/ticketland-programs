#![cfg(feature = "test-bpf")]

use test_context::{test_context, futures};
use anchor_lang::{prelude::*};
use solana_sdk::{
  signature::{Signer, Keypair},
};
use solana_program_test::{tokio};
use common_test::{
  event_registry::{
    runner::Runner as EventRegistryRunner,
  },
  ticket_sale::{
    runner::Runner as TicketSaleRunner,
    pda as ticket_sale_pda,
  },
  test_context::TestContext,
};
use common::{
  state::{
    ticket_type::{TicketType, SeatRange},
    sale_type::SaleType,
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
  ticket_sale_runner: &mut TicketSaleRunner,
  ticket_sale_program_state: Pubkey,
  event_capacity: Pubkey,
  event_id: [u8; 32],
  event_organizer: &Keypair,
  deposit_token: Pubkey,
) -> Vec<TicketType> {
  // ticket type 1 includes seats 0, 1, 2, 5, 6, 7
  let mt_type_1 = ticket_sale_runner.create_ticket_type_mt(vec![(0, 2), (5, 7)], 10);
  // ticket type 3 includes seats 3, 4, 8, 9
  let mt_type_2 = ticket_sale_runner.create_ticket_type_mt(vec![(3, 4), (8, 9)], 10);

  let ticket_types;
  
  {
    let mut pt = ticket_sale_runner.pt.lock().await;
    let now = pt.get_clock().await.unix_timestamp;

    ticket_types = vec![
      TicketType {
        n_tickets: 4,
        sale_type: SaleType::Free,
        sale_start_time: now + 10, // 10 seconds
        sale_end_time: now + 10 + 10,
        merkle_root: mt_type_1.root().unwrap(),
        seat_range: SeatRange {l: 0, r: 10_000},
      },
      TicketType {
        n_tickets: 6,
        sale_type: SaleType::DutchAuction {
          start_price: 150,
          end_price: 110,
          curve_length: 200 * 60,
          drop_interval: 20 * 60,
        },
        sale_start_time: now + 15, // 15 seconds
        sale_end_time: now + 15 + 10,
        merkle_root: mt_type_2.root().unwrap(),
        seat_range: SeatRange {l: 10_001, r: 20_000},
      },
    ];  
  }

  let _ = event_registry_runner.create_event(
    event_registry_state,
    event_capacity,
    ticket_sale_program_state,
    event_id,
    deposit_token,
    deposit_token,
    &event_organizer,
    10, // num of tickets
    100,
		1000,
		ticket_types.clone(),
  ).await;

  // Create the NFT as well
  let _ = event_registry_runner.create_event_nft(
    event_registry_state,
    event_id,
    &event_organizer,
    "Ticket Land Coolest Event".to_owned(),
    "TICKT".to_owned(),
    "https://ticketland.io".to_owned(),
  ).await;

  ticket_types
}

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_create_a_new_sale(ctx: &mut TestContext) {
  let (
    event_registry_state,
    ticket_sale_state,
    _,
  ) = init(ctx).await;

  let event_registry_runner = &mut ctx.event_registry_runner;
  let ticket_sale_runner = &mut ctx.ticket_sale_runner;
  let event_capacity = event_registry_runner.create_event_capacity_account(10.0).await;
  let event_id: [u8; 32] = "85ac6394e04a4b3c8ccd7e2772cb14b4".to_owned().into_bytes().try_into().unwrap();
  let event_organizer = event_registry_runner.get_participant(1);
  let deposit_token = event_registry_runner.deposit_tokens[2];

  let ticket_types = custom_create_event(
    event_registry_runner,
    event_registry_state.pubkey(),
    ticket_sale_runner,
    ticket_sale_state.pubkey(),
    event_capacity,
    event_id,
    &event_organizer,
    deposit_token,
  ).await;

  // init the ticker sale program first
  let ticket_sale_runner = &mut ctx.ticket_sale_runner;
  let ticket_type_index = 0;
  let event_id: [u8; 32] = "85ac6394e04a4b3c8ccd7e2772cb14b4".to_owned().into_bytes().try_into().unwrap();

  // Create a new ticket sale for the first ticket type
  let result = ticket_sale_runner.create_sale(
    ticket_sale_state.pubkey(),
    event_registry_state.pubkey(),
    event_id,
    &event_organizer,
    ticket_type_index,
  ).await;

  assert!(result.is_ok());

  {
    let mut pt = ticket_sale_runner.pt.lock().await;
    let ticket_sale_state = ticket_sale_pda::ticket_sale_state(
      &ticket_sale_state.pubkey(),
      ticket_type_index,
      event_id,
    ).0;
    let sale_data = pt.get_account::<ticket_sale::account_data::sale::Sale>(ticket_sale_state).await;
    
    assert_eq!(sale_data.event_id, event_id);
    assert_eq!(sale_data.ticket_type_index, ticket_type_index);
    assert_eq!(sale_data.ticket_type, ticket_types[0].clone());
  }

  // Create a new ticket sale for the second ticket type
  let ticket_type_index = 1;

  let result = ticket_sale_runner.create_sale(
    ticket_sale_state.pubkey(),
    event_registry_state.pubkey(),
    event_id,
    &event_organizer,
    ticket_type_index,
  ).await;

  assert!(result.is_ok());

  {
    let mut pt = ticket_sale_runner.pt.lock().await;
    let (ticket_sale_state, ticket_sale_state_bump) = ticket_sale_pda::ticket_sale_state(
      &ticket_sale_state.pubkey(),
      ticket_type_index,
      event_id,
    );
    let sale_data = pt.get_account::<ticket_sale::account_data::sale::Sale>(ticket_sale_state).await;

    assert_eq!(sale_data.bump, ticket_sale_state_bump);
    assert_eq!(sale_data.event_id, event_id);
    assert_eq!(sale_data.ticket_type_index, ticket_type_index);
    assert_eq!(sale_data.ticket_type, ticket_types[1].clone());
  }
}
