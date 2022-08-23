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
};

async fn init(ctx: &mut TestContext) -> (Keypair, Keypair, Keypair, Keypair) {
  let event_registry_state = Keypair::new();
  let ticket_sale_state = Keypair::new();
  let ticket_nft_state = Keypair::new();
  let secondary_market_state = Keypair::new();
  let event_registry_runner = &mut ctx.event_registry_runner;
  let ticket_sale_runner = &mut ctx.ticket_sale_runner;
  let ticket_nft_runner = &mut ctx.ticket_nft_runner;
  let secondary_market_runner = &mut ctx.secondary_market_runner;

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
    secondary_market_state.pubkey(),
  ).await;

  secondary_market_runner.initialize(
    &secondary_market_state,
    event_registry_state.pubkey(),
    ticket_sale_state.pubkey(),
    ticket_nft_state.pubkey(),
    500, // 5%
  ).await;

  (
    event_registry_state,
    ticket_sale_state,
    ticket_nft_state,
    secondary_market_state,
  )
}

async fn custom_create_event(
  ctx: &mut TestContext,
  event_registry_state: Pubkey,
  ticket_sale_program_state: Pubkey,
  event_id: [u8; 32],
) -> (Vec<TicketType>,) {
  let event_registry_runner = &mut ctx.event_registry_runner;
  let ticket_sale_runner = &mut ctx.ticket_sale_runner;
  let event_capacity = event_registry_runner.create_event_capacity_account().await;

  // ticket type 1 includes seats 0, 1, 2, 5, 6, 7
  let mt_type_1 = ticket_sale_runner.create_ticket_type_mt(vec![(0, 2), (5, 7)]);
  // ticket type 2 includes seats 3, 4, 8, 9
  let mt_type_2 = ticket_sale_runner.create_ticket_type_mt(vec![(3, 4), (8, 9)]);

  let ticket_types;
  {
    let mut pt = ticket_sale_runner.pt.lock().await;
    let now = pt.get_clock().await.unix_timestamp;

    ticket_types = vec![
      TicketType {
        n_tickets: 4,
        sale_type: SaleType::FixedPrice {amount: sol_to_lamports(1_f64)},
        sale_start_time: now + 1, // 10 seconds
        sale_end_time: now + 1 + 10,
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
        sale_start_time: now + 2, // 10 seconds from now,
        sale_end_time: now + 2 + 10,
        merkle_root: mt_type_2.root().unwrap(),
        seat_range: SeatRange {l: 10_001, r: 20_000},
      },
    ];
  }

  let event_organizer = event_registry_runner.get_participant(1);
  let deposit_token = event_registry_runner.deposit_tokens[2];

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

  (ticket_types,)
}

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_fail_if_sale_finished(ctx: &mut TestContext) {
  let (
    event_registry_state,
    ticket_sale_state,
    ticket_nft_state,
    secondary_market_state,
  ) = init(ctx).await;

  let event_id: [u8; 32] = "85ac6394e04a4b3c8ccd7e2772cb14b4".to_owned().into_bytes().try_into().unwrap();
  
  let (ticket_types,) = custom_create_event(
    ctx,
    event_registry_state.pubkey(),
    ticket_sale_state.pubkey(),
    event_id,
  ).await;

  // move to the end of sale
  {
    let ticket_sale_runner = &mut ctx.ticket_sale_runner;
    let mut pt = ticket_sale_runner.pt.lock().await;
    pt.advance_clock_past_timestamp(ticket_types[0].sale_end_time).await;
  }
}
