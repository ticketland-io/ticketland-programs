#![cfg(feature = "test-bpf")]
use anchor_lang::{
  prelude::{
    Pubkey,
  },
  AccountDeserialize,
};
use test_context::{test_context, futures};
use solana_test_utils::{
  spl::Spl,
  serialization::deser_zero_account,
};
use solana_sdk::{
  signature::{Signer, Keypair},
  native_token::sol_to_lamports,
};
use anchor_spl::{
  token::{Mint as TokenMint, TokenAccount},
};
use anchor_metaplex::{
  mpl_token_metadata::{
    deser::meta_deser,
    pda::{
      find_metadata_account,
    },
  },
};
use solana_program_test::{tokio};
use common::{
  utils::bitmap,
  state::{
    ticket_type::TicketType,
    sale_type::SaleType,
  },
};
use common_test::{
  test_context::TestContext,
  event_registry::{
    runner::Runner as EventRegistryRunner,
  },
  ticket_sale::{
    runner::Runner as TicketSaleRunner,
    pda as TickerSalePda,
  },
  ticket_nft::{
    pda as TicketNftPda,
  },
};
use ticket_sale::{
  account_data::event_capacity::{EventCapacity, MAX_VENUE_CAPACITY},
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
  let deposit_token = event_registry_runner.deposit_tokens[2];

  // ticket type 1 includes seats 0, 1, 2, 5, 6, 7
  let mt_type_1 = ticket_sale_runner.create_ticket_type_mt(vec![(0, 2), (5, 7)]);
  // ticket type 3 includes seats 3, 4, 8, 9
  let mt_type_2 = ticket_sale_runner.create_ticket_type_mt(vec![(3, 4), (8, 9)]);

  let ticket_types = vec![
    TicketType {
      n_tickets: 4,
      sale_type: SaleType::FixedPrice(sol_to_lamports(1_f64)),
      sale_start_time: 10,
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
      sale_start_time: 15,
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
  let purchase_token = event_registry_runner.deposit_tokens[2];

  // move to the start of sale
  {
    let mut pt = ticket_sale_runner.pt.lock().await;
    pt.context.warp_to_slot(11).unwrap();
  }

  let event_organizer_funds_before;
  let treasury_funds_before;
  {
    let mut pt = ticket_sale_runner.pt.lock().await;
    let treasury = ticket_sale_runner.treasury.pubkey();

    event_organizer_funds_before = pt.context.banks_client.get_account(event_organizer.pubkey()).await.unwrap().unwrap();
    treasury_funds_before = pt.context.banks_client.get_account(treasury).await.unwrap().unwrap();
  }

  let seat_index = 0;
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
    seat_index,
		TicketSaleRunner::dummy_seat_name(0),
		mt_type_1.proof(&[0]), // proof path for leaf 0
  ).await;

  assert!(result.is_ok());

  // funds are transferred
  {
    let mut pt = ticket_sale_runner.pt.lock().await;
    let treasury = ticket_sale_runner.treasury.pubkey();
    
    let event_organizer_funds_after = pt.context.banks_client.get_account(event_organizer.pubkey()).await.unwrap().unwrap();
    assert_eq!(event_organizer_funds_after.lamports - event_organizer_funds_before.lamports, sol_to_lamports(0.95_f64));

    // 5% feeds go to treasury
    let treasury_funds_after = pt.context.banks_client.get_account(treasury).await.unwrap().unwrap();
    assert_eq!(treasury_funds_after.lamports - treasury_funds_before.lamports, sol_to_lamports(0.05_f64));
  }

  let ticket_nft = TicketNftPda::ticket_nft(&ticket_nft_state.pubkey(), &ticket_buyer.pubkey(), event_id).0;

  // ticket nft Mint account and Metaplex metadata
  {
    let mut pt = ticket_sale_runner.pt.lock().await;
    let nft_authority = TicketNftPda::nft_authority(&ticket_nft_state.pubkey()).0;
    let ticket_nft_data = pt.context.banks_client.get_account(ticket_nft).await.unwrap().unwrap();
    let ticket_nft_data = TokenMint::try_deserialize_unchecked(&mut &ticket_nft_data.data[..]).unwrap();
    
    // mint authority is transferred to the master edition when the latter is created
    assert_eq!(ticket_nft_data.mint_authority.unwrap(), nft_authority);
    assert_eq!(ticket_nft_data.supply, 1);

    // Assert the ATA account. This account is the holder of the NFT and is owned by the CPI Authority PDA
    // controlled by the ticket sale program.
    let ticket_sale_cpi_authority = TickerSalePda::cpi_authority(&ticket_sale_state.pubkey()).0;
    let ticket_nft_ata = Spl::get_associated_token_address(&ticket_sale_cpi_authority, &ticket_nft);
    let ticket_nft_ata_data = pt.context.banks_client.get_account(ticket_nft_ata).await.unwrap().unwrap();
    let ticket_nft_ata_data = TokenAccount::try_deserialize_unchecked(&mut &ticket_nft_ata_data.data[..]).unwrap();

    assert_eq!(ticket_nft_ata_data.mint, ticket_nft);
    assert_eq!(ticket_nft_ata_data.owner, ticket_sale_cpi_authority);
    assert_eq!(ticket_nft_ata_data.amount, 1);

    // metaplex
    let metadata = find_metadata_account(&ticket_nft).0;
    let account = pt.context.banks_client.get_account(metadata).await.unwrap().unwrap();
    let metadata = meta_deser(&mut &account.data[..]).unwrap();

    assert_eq!(metadata.update_authority, nft_authority);
    assert_eq!(metadata.mint, ticket_nft);
    assert_eq!(metadata.collection, None);
    assert_eq!(metadata.data.name.trim_matches(char::from(0)), TicketSaleRunner::dummy_seat_name(0));
    assert_eq!(metadata.data.symbol.trim_matches(char::from(0)), "TICKT".to_owned());
    assert_eq!(metadata.data.uri.trim_matches(char::from(0)), "https://ticketland.io".to_owned());
    assert_eq!(metadata.data.seller_fee_basis_points, 0);
  }

  // Check out custom Ticket Metadata
  {
    let mut pt = ticket_sale_runner.pt.lock().await;
    let ticket_metadata = TicketNftPda::ticket_metadata(&ticket_nft_state.pubkey(), &ticket_nft).0;
    let ticket_metadata = pt.get_account::<ticket_nft::account_data::ticket_metadata::TicketMetadata>(ticket_metadata).await;
    let metaplex_metadata = find_metadata_account(&ticket_nft).0;

    assert_eq!(ticket_metadata.event_id, event_id);
    assert_eq!(ticket_metadata.metadata, metaplex_metadata);
    assert_eq!(ticket_metadata.owner, ticket_buyer.pubkey());
    assert_eq!(ticket_metadata.attended, false);
  }

  // Ticket sale state is updated
  {
    let mut pt = ticket_sale_runner.pt.lock().await;
    let ticket_metadata = pt.get_account::<ticket_sale::account_data::state::State>(ticket_sale_state.pubkey()).await;

    assert_eq!(ticket_metadata.total_sold, 1);
  }

  // Event capacity is updated
  {
    let mut pt = ticket_sale_runner.pt.lock().await;
    let event_capacity = pt.context.banks_client.get_account(event_capacity).await.unwrap().unwrap();
    let event_capacity = deser_zero_account::<EventCapacity>(&event_capacity.data);

    assert!(bitmap::is_set::<MAX_VENUE_CAPACITY>(seat_index, &event_capacity.seats));
    assert_eq!(event_capacity.available_tickets, 9);
    assert_eq!(event_capacity.is_initialized, true);
    assert_eq!(event_capacity.event_id, event_id);
  }
}
