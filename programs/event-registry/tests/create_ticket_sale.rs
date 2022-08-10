#![cfg(feature = "test-bpf")]

use test_context::{test_context, futures};
use anchor_lang::{prelude::*};
use solana_sdk::{
  signature::{Signer, Keypair},
};
use solana_test_utils::{
  utils::{to_base},
};
use solana_program_test::{tokio};
use common_test::{
  event_registry::{
    runner::Runner,
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

async fn custom_create_event(
  skip_init: bool,
  runner: &mut Runner,
  ticket_sale_runner: &mut TicketSaleRunner,
  state: &Keypair,
  event_capacity: Pubkey,
  event_id: [u8; 32],
  event_organizer: &Keypair,
  deposit_token_idx: usize,
) -> Vec<TicketType> {
  if !skip_init {
    runner.initialize(
      &state,
      1_000, // 10%
    ).await;
  }
  
  let ticket_sale_program_state = initialize_ticket_sale(ticket_sale_runner, state.pubkey()).await;
  let deposit_token = runner.deposit_tokens[deposit_token_idx];

  let ticket_types = vec![
    TicketType {
      n_tickets: 50_000,
      sale_type: SaleType::FixedPrice {amount: to_base(100, 6)},
      sale_start_time: 50,
      merkle_root: [0; 32],
      seat_range: SeatRange {l: 0, r: 10_000},
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
      seat_range: SeatRange {l: 10_001, r: 20_000},
    },
  ];

  let _ = runner.create_event(
    state.pubkey(),
    event_capacity,
    ticket_sale_program_state,
    event_id,
    deposit_token,
    deposit_token,
    &event_organizer,
    100_000,
    100,
		1000,
		ticket_types.clone(),
  ).await;

  // Create the NFT as well
  let _ = runner.create_event_nft(
    state.pubkey(),
    event_id,
    &event_organizer,
    "Ticket Land Coolest Event".to_owned(),
    "TICKT".to_owned(),
    "https://ticketland.io".to_owned(),
  ).await;

  ticket_types
}

async fn initialize_ticket_sale(
  ticket_sale_runner: &mut TicketSaleRunner,
  event_registry_state: Pubkey
) -> Pubkey {
  let ticket_sale_state = Keypair::new();

  ticket_sale_runner.initialize(
    &ticket_sale_state,
    event_registry_state,
  ).await;

  ticket_sale_state.pubkey()
}

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_create_a_new_sale_by_calling_the_ticket_sale_program(ctx: &mut TestContext) {
  let runner = &mut ctx.event_registry_runner;
  let ticket_sale_runner = &mut ctx.ticket_sale_runner;
  let state = Keypair::new();
  let event_capacity = runner.create_event_capacity_account().await;
  let event_id: [u8; 32] = "85ac6394e04a4b3c8ccd7e2772cb14b4".to_owned().into_bytes().try_into().unwrap();
  let event_organizer = runner.get_participant(1);
  
  let ticket_types = custom_create_event(
    false,
    runner,
    ticket_sale_runner,
    &state,
    event_capacity,
    event_id,
    &event_organizer,
    0,
  ).await;

  // init the ticker sale program first
  let ticket_sale_runner = &mut ctx.ticket_sale_runner;
  let ticket_sale_program_state = initialize_ticket_sale(ticket_sale_runner, state.pubkey()).await;
  let ticket_type_index = 0;
  let event_id: [u8; 32] = "85ac6394e04a4b3c8ccd7e2772cb14b4".to_owned().into_bytes().try_into().unwrap();

  // Create a new ticket sale for the first ticket type
  let result = runner.create_ticket_sale(
    state.pubkey(),
    event_id,
    &event_organizer,
    ticket_sale_program_state,
    ticket_type_index,
  ).await;

  assert!(result.is_ok());

  {
    let mut pt = runner.pt.lock().await;
    let ticket_sale_state = ticket_sale_pda::ticket_sale_state(
      &ticket_sale_program_state,
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

  let result = runner.create_ticket_sale(
    state.pubkey(),
    event_id,
    &event_organizer,
    ticket_sale_program_state,
    ticket_type_index,
  ).await;

  assert!(result.is_ok());

  {
    let mut pt = runner.pt.lock().await;
    let (ticket_sale_state, ticket_sale_state_bump) = ticket_sale_pda::ticket_sale_state(
      &ticket_sale_program_state,
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
