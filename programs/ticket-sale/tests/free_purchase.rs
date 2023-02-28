#![cfg(feature = "test-bpf")]
use anchor_lang::{
  prelude::{
    Pubkey,
    Result as AnchorResult,
  },
  AccountDeserialize,
};
use test_context::{test_context};
use solana_test_utils::{
  spl::Spl,
  merkle_tree::MerkleTree,
};
use solana_sdk::{
  signature::{Signer, Keypair},
};
use anchor_metaplex::{
  mpl_token_metadata::{
    pda::{
      find_metadata_account,
    },
  },
};
use anchor_spl::{
  token::{TokenAccount},
};
use solana_program_test::{tokio};
use common::{
  utils::bitmap,
  state::{
    ticket_type::{TicketType, SeatRange},
    sale_type::SaleType,
  },
};
use common_test::{
  test_context::TestContext,
  event_registry::{
    runner::Runner as EventRegistryRunner,
    pda as EventRegistryPda,
  },
  ticket_sale::{
    runner::Runner as TicketSaleRunner,
    pda as TickerSalePda,
    error::Error,
  },
  ticket_nft::{
    pda as TicketNftPda,
  },
};
use ticket_sale::{
  account_data::event_capacity::EventCapacity,
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
  event_id: [u8; 32],
  event_organizer: &Keypair,
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
}

async fn setup(ctx: &mut TestContext) -> (Keypair, Keypair, Keypair, Keypair, Vec<TicketType>, [u8; 32], MerkleTree, Pubkey)  {
  let (
    event_registry_state,
    ticket_sale_state,
    ticket_nft_state,
  ) = init(ctx).await;

  let event_registry_runner = &mut ctx.event_registry_runner;
  let ticket_sale_runner = &mut ctx.ticket_sale_runner;
  let event_capacity = event_registry_runner.create_event_capacity_account(10.0).await;
  let event_id: [u8; 32] = "85ac6394e04a4b3c8ccd7e2772cb14b4".to_owned().into_bytes().try_into().unwrap();
  let event_organizer = event_registry_runner.get_participant(1);
  let deposit_token = event_registry_runner.deposit_tokens[2];

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
        name: "Basic".to_string(),
        n_tickets: 4,
        sale_type: SaleType::Free,
        sale_start_time: now + 20, // 20 seconds
        sale_end_time: now + 20 + 20,
        merkle_root: mt_type_1.root().unwrap(),
        seat_range: SeatRange {l: 0, r: 10_000},
      },
      TicketType {
        name: "VIP".to_string(),
        n_tickets: 6,
        sale_type: SaleType::DutchAuction {
          start_price: 150,
          end_price: 110,
          curve_length: 200 * 60,
          drop_interval: 20 * 60,
        },
        sale_start_time: now + 25, // 25 seconds
        sale_end_time: now + 25 + 10,
        merkle_root: mt_type_2.root().unwrap(),
        seat_range: SeatRange {l: 10_001, r: 20_000},
      },
    ];  
  }

  custom_create_event(
    event_registry_runner,
    event_registry_state.pubkey(),
    ticket_sale_state.pubkey(),
    event_capacity,
    event_id,
    &event_organizer,
    deposit_token,
    &ticket_types,
  ).await;
  
  // Create a new ticket sale for the first ticket type
  let _ = ticket_sale_runner.create_sale(
    ticket_sale_state.pubkey(),
    event_registry_state.pubkey(),
    event_id,
    &event_organizer,
    0, // ticket_type_index
  ).await;

  (
    event_organizer,
    event_registry_state,
    ticket_sale_state,
    ticket_nft_state,
    ticket_types,
    event_id,
    mt_type_1,
    event_capacity,
  )
}

async fn setup_reservation(ctx: &mut TestContext, ticket_buyer: &Keypair, recipient: Pubkey, should_expire: bool) -> (AnchorResult<()>, Pubkey) {
  let (
    event_organizer,
    event_registry_state,
    ticket_sale_state,
    ticket_nft_state,
    ticket_types,
    event_id,
    mt_type_1,
    event_capacity,
  ) = setup(ctx).await;

  // move to the start of sale
  {
    let mut pt = ctx.ticket_sale_runner.pt.lock().await;
    pt.advance_clock_past_timestamp(ticket_types[0].sale_start_time).await;
  }

  let seat_index = 0;
  // verify the seat
  {
    let result = ctx.ticket_sale_runner.verify_seat(
      &ticket_buyer,
      ticket_sale_state.pubkey(),
      event_id,
      0, // ticket_type_index
      seat_index,
      TicketSaleRunner::dummy_seat_name(0),
      mt_type_1.proof(&[0]), // proof path for leaf 0
    ).await;

    assert!(result.is_ok());
  }

  let seat_name = TicketSaleRunner::dummy_seat_name(0);
  // operator reserves this seat
  {
    let operator = ctx.ticket_sale_runner.get_participant(7);
    let result = ctx.ticket_sale_runner.reserve_seat(
      ticket_sale_state.pubkey(),
      &operator,
      recipient,
      event_id,
      0, // ticket_type_index
      seat_index,
      seat_name.clone(),
      10
    ).await;

    assert!(result.is_ok());
  }

  if should_expire {
    let mut pt = ctx.ticket_sale_runner.pt.lock().await;
    pt.advance_clock_by_slots(11).await;
  }

  let result = ctx.ticket_sale_runner.free_purchase(
    &ticket_buyer,
    event_registry_state.pubkey(),
    ticket_sale_state.pubkey(),
    event_capacity,
    event_organizer.pubkey(),
    ticket_nft_state.pubkey(),
    event_id,
    0, // ticket_type_index
    seat_index,
		TicketSaleRunner::dummy_seat_name(0),
  ).await;

  let seat_reservation = TickerSalePda::seat_reservation(&ticket_sale_state.pubkey(), seat_index, &seat_name).0;

  (result, seat_reservation)
}

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_allow_ticket_buyer_to_purchase_ticket_for_free(ctx: &mut TestContext) {
  let (
    event_organizer,
    event_registry_state,
    ticket_sale_state,
    ticket_nft_state,
    ticket_types,
    event_id,
    mt_type_1,
    event_capacity,
  ) = setup(ctx).await;

  let ticket_buyer = ctx.event_registry_runner.get_participant(2);

  // move to the start of sale
  {
    let mut pt = ctx.ticket_sale_runner.pt.lock().await;
    pt.advance_clock_past_timestamp(ticket_types[0].sale_start_time).await;
  }

  let seat_index = 0;
  // verify the seat
  {
    let result = ctx.ticket_sale_runner.verify_seat(
      &ticket_buyer,
      ticket_sale_state.pubkey(),
      event_id,
      0, // ticket_type_index
      seat_index,
      TicketSaleRunner::dummy_seat_name(0),
      mt_type_1.proof(&[0]), // proof path for leaf 0
    ).await;

    assert!(result.is_ok());
  }

  let ticket_type_index = 0;
  let result = ctx.ticket_sale_runner.free_purchase(
    &ticket_buyer,
    event_registry_state.pubkey(),
    ticket_sale_state.pubkey(),
    event_capacity,
    event_organizer.pubkey(),
    ticket_nft_state.pubkey(),
    event_id,
    ticket_type_index,
    seat_index,
		TicketSaleRunner::dummy_seat_name(0),
  ).await;

  assert!(result.is_ok());

  let ticket_nft = TicketNftPda::ticket_nft(&ticket_nft_state.pubkey(), seat_index, event_id, ticket_type_index).0;

  // ticket nft Mint account
  {
    let mut pt = ctx.ticket_sale_runner.pt.lock().await;

    // Assert the ATA account. This account is the holder of the NFT and is owned by the CPI Authority PDA
    // controlled by the ticket sale program.
    let ticket_sale_cpi_authority = TickerSalePda::cpi_authority(&ticket_sale_state.pubkey()).0;
    let ticket_nft_ata = Spl::get_associated_token_address(&ticket_sale_cpi_authority, &ticket_nft);
    let ticket_nft_ata_data = pt.context.banks_client.get_account(ticket_nft_ata).await.unwrap().unwrap();
    let ticket_nft_ata_data = TokenAccount::try_deserialize_unchecked(&mut &ticket_nft_ata_data.data[..]).unwrap();

    assert_eq!(ticket_nft_ata_data.mint, ticket_nft);
    assert_eq!(ticket_nft_ata_data.owner, ticket_sale_cpi_authority);
    assert_eq!(ticket_nft_ata_data.amount, 1);
  }

  // Check out custom Ticket Metadata
  {
    let mut pt = ctx.ticket_sale_runner.pt.lock().await;
    let ticket_metadata = TicketNftPda::ticket_metadata(&ticket_nft_state.pubkey(), &ticket_nft).0;
    let ticket_metadata = pt.get_account::<ticket_nft::account_data::ticket_metadata::TicketMetadata>(ticket_metadata).await;
    let sale = TickerSalePda::ticket_sale_state(&ticket_sale_state.pubkey(), 0, event_id).0;
    let event_nft = EventRegistryPda::event_nft(&event_registry_state.pubkey(), event_id).0;

    assert_eq!(ticket_metadata.mint, ticket_nft);
    assert_eq!(ticket_metadata.collection, find_metadata_account(&event_nft).0);
    assert_eq!(ticket_metadata.name.trim_matches(char::from(0)), TicketSaleRunner::dummy_seat_name(0));
    assert_eq!(ticket_metadata.event_id, event_id);
    assert_eq!(ticket_metadata.owner, ticket_buyer.pubkey());
    assert_eq!(ticket_metadata.seat_index, seat_index);
    assert_eq!(ticket_metadata.price_sold, 0);
    assert_eq!(ticket_metadata.sale, sale);
    assert_eq!(ticket_metadata.attended, false);
  }

  // Ticket sale state is updated
  {
    let mut pt = ctx.ticket_sale_runner.pt.lock().await;
    let ticket_metadata = pt.get_account::<ticket_sale::account_data::state::State>(ticket_sale_state.pubkey()).await;

    assert_eq!(ticket_metadata.total_sold, 1);
  }

  // Event capacity is updated
  {
    let mut pt = ctx.ticket_sale_runner.pt.lock().await;
    let event_capacity = pt.get_account::<EventCapacity>(event_capacity).await;
    let available_tickets = event_capacity.available_tickets;

    assert!(bitmap::is_set(seat_index, &event_capacity.seats));
    assert_eq!(available_tickets, 9);
    assert_eq!(event_capacity.is_initialized, true);
    assert_eq!(event_capacity.event_id, event_id);
  }
}

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_fail_if_seat_was_not_verified(ctx: &mut TestContext) {
  let (
    event_organizer,
    event_registry_state,
    ticket_sale_state,
    ticket_nft_state,
    ticket_types,
    event_id,
    _,
    event_capacity,
  ) = setup(ctx).await;

  let ticket_buyer = ctx.event_registry_runner.get_participant(2);

  // move to the start of sale
  {
    let mut pt = ctx.ticket_sale_runner.pt.lock().await;
    pt.advance_clock_past_timestamp(ticket_types[0].sale_start_time).await;
  }

  let result = ctx.ticket_sale_runner.free_purchase(
    &ticket_buyer,
    event_registry_state.pubkey(),
    ticket_sale_state.pubkey(),
    event_capacity,
    event_organizer.pubkey(),
    ticket_nft_state.pubkey(),
    event_id,
    0, // ticket_type_index
    0,
		TicketSaleRunner::dummy_seat_name(0),
  ).await;

  // Fails with Anchor Error Code: AccountNotInitialized. Error Number: 3012
  assert!(!result.is_ok());
}

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_close_the_seat_verification_account(ctx: &mut TestContext) {
  let (
    event_organizer,
    event_registry_state,
    ticket_sale_state,
    ticket_nft_state,
    ticket_types,
    event_id,
    mt_type_1,
    event_capacity,
  ) = setup(ctx).await;

  let ticket_buyer = ctx.event_registry_runner.get_participant(2);

  // move to the start of sale
  {
    let mut pt = ctx.ticket_sale_runner.pt.lock().await;
    pt.advance_clock_past_timestamp(ticket_types[0].sale_start_time).await;
  }

  let seat_index = 0;
  // verify the seat
  {
    let result = ctx.ticket_sale_runner.verify_seat(
      &ticket_buyer,
      ticket_sale_state.pubkey(),
      event_id,
      0, // ticket_type_index
      seat_index,
      TicketSaleRunner::dummy_seat_name(0),
      mt_type_1.proof(&[0]), // proof path for leaf 0
    ).await;

    assert!(result.is_ok());
  }

  let result = ctx.ticket_sale_runner.free_purchase(
    &ticket_buyer,
    event_registry_state.pubkey(),
    ticket_sale_state.pubkey(),
    event_capacity,
    event_organizer.pubkey(),
    ticket_nft_state.pubkey(),
    event_id,
    0, // ticket_type_index
    seat_index,
		TicketSaleRunner::dummy_seat_name(0),
  ).await;

  assert!(result.is_ok());

  let seat_verification = TickerSalePda::seat_verification(&ticket_sale_state.pubkey(), seat_index, &TicketSaleRunner::dummy_seat_name(0)).0;
  let mut pt = ctx.ticket_sale_runner.pt.lock().await;
  let account = pt.context.banks_client.get_account(seat_verification).await.unwrap();
  assert!(account.is_none());
}

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_fail_if_seat_is_reserved(ctx: &mut TestContext) {
  let recipient = ctx.event_registry_runner.get_participant(2);
  let ticket_buyer = ctx.event_registry_runner.get_participant(3);
  let (result, _) = setup_reservation(ctx, &ticket_buyer, recipient.pubkey(), false).await;

  Error::assert_err(result, ticket_sale::utils::program_error::ErrorCode::SeatReserved);
}

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_not_fail_if_seat_reservation_has_expired(ctx: &mut TestContext) {
  let recipient = ctx.event_registry_runner.get_participant(2);
  let ticket_buyer = ctx.event_registry_runner.get_participant(3);
  let (result, _) = setup_reservation(ctx, &ticket_buyer, recipient.pubkey(), true).await;
  
  assert!(result.is_ok());
}

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_not_fail_if_seat_reserved_for_the_current_user(ctx: &mut TestContext) {
  let ticket_buyer = ctx.event_registry_runner.get_participant(2);
  let (result, _) = setup_reservation(ctx, &ticket_buyer, ticket_buyer.pubkey(), false).await;

  assert!(result.is_ok());
}

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_close_the_seat_reservation_account(ctx: &mut TestContext) {
  let ticket_buyer = ctx.event_registry_runner.get_participant(2);
  let (_, seat_reservation) = setup_reservation(ctx, &ticket_buyer, ticket_buyer.pubkey(), false).await;

  let mut pt = ctx.ticket_sale_runner.pt.lock().await;
  let account = pt.context.banks_client.get_account(seat_reservation).await.unwrap();
  assert!(account.is_none());
}
