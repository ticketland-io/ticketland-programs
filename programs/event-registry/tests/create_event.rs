#![cfg(feature = "test-bpf")]

use test_context::{test_context, futures};
use anchor_lang::{
  prelude::*,
};
use solana_sdk::{
  system_program,
  signature::{Signer, Keypair},
  native_token::sol_to_lamports,
};
use solana_test_utils::{
  utils::{to_base},
};
use anchor_metaplex::{
  mpl_token_metadata::{
    deser::meta_deser,
    pda::{
      find_metadata_account,
      find_master_edition_account,
    },
  },
};
use solana_program_test::{tokio};
use common_test::{
  ticket_sale::{
    runner::Runner as TicketSaleRunner,
    pda as ticket_sale_pda,
  },
  event_registry::{
    pda,
    runner::Runner,
    error::Error,
  },
  test_context::TestContext,
};
use anchor_lang::{
  prelude::Result as AnchorResult,
};
use anchor_spl::{
  token::{Mint as TokenMint, TokenAccount},
};
use common::{
  state::{
    ticket_type::TicketType,
    sale_type::SaleType,
  },
};
use ticket_sale::account_data::event_capacity::{
  EventCapacity,
  SPACE_MARGIN as event_capacity_space_margin,
};

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

async fn custom_create_event(
  skip_init: bool,
  runner: &mut Runner,
  ticket_sale_runner: &mut TicketSaleRunner,
  state: &Keypair,
  event_capacity: Pubkey,
  event_id: u64,
  event_organizer: &Keypair,
  deposit_token_idx: usize,
) -> AnchorResult<()> {
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
      n_tickets: 1000,
      sale_type: SaleType::FixedPrice(to_base(100, 6)),
      sale_start_time: 50,
      merkle_root: [0; 32],
    },
    TicketType {
      n_tickets: 1000,
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

  runner.create_event(
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
		"Ticket Land Coolest Event".to_owned(),
		"TICKT".to_owned(),
		"https://ticketland.io".to_owned(),
  ).await

}

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_create_a_new_event(ctx: &mut TestContext) {
  let runner = &mut ctx.event_registry_runner;
  let ticket_sale_runner = &mut ctx.ticket_sale_runner;
  let state = Keypair::new();
  let event_capacity = runner.create_event_capacity_account().await;
  let event_id = 0;
  let event_organizer = runner.get_participant(1);
  
  let result = custom_create_event(
    false,
    runner,
    ticket_sale_runner,
    &state,
    event_capacity,
    event_id,
    &event_organizer,
    0,
  ).await;

  assert!(result.is_ok());

  // Assert event state
  {
    let mut pt = runner.pt.lock().await;
    let event = pda::event(&state.pubkey(), event_id).0;
    let event_data = pt.get_account::<event_registry::account_data::event::Event>(event).await;

    assert_eq!(event_data.id, event_id);
    assert_eq!(event_data.event_capacity, event_capacity);
    assert_eq!(event_data.n_tickets, 100_000);
    assert_eq!(event_data.start_time, 100);
    assert_eq!(event_data.end_time, 1000);
    assert_eq!(event_data.event_organizer, event_organizer.pubkey());
  }

  // Assert state
  {
    let mut pt = runner.pt.lock().await;
    let state_data = pt.get_account::<event_registry::account_data::state::State>(state.pubkey()).await;

    // number of events increased by one
    assert_eq!(state_data.n_events, 1);
  }
  
  let event_nft = pda::event_nft(&state.pubkey(), event_id).0;

  // Assert event nft metadata
  {
    let metadata = find_metadata_account(&event_nft).0;
    let mut pt = runner.pt.lock().await;
    let account = pt.context.banks_client.get_account(metadata).await.unwrap().unwrap();
    let metadata = meta_deser(&mut &account.data[..]).unwrap();

    assert_eq!(metadata.update_authority, pda::event_nft_authority(&state.pubkey()).0);
    assert_eq!(metadata.mint, event_nft);
    assert_eq!(metadata.collection, None);
    assert_eq!(metadata.data.name.trim_matches(char::from(0)), "Ticket Land Coolest Event".to_owned());
    assert_eq!(metadata.data.symbol.trim_matches(char::from(0)), "TICKT".to_owned());
    assert_eq!(metadata.data.uri.trim_matches(char::from(0)), "https://ticketland.io".to_owned());
    assert_eq!(metadata.data.seller_fee_basis_points, 1000);
  }

  // Assert the token mint account
  {
    let mut pt = runner.pt.lock().await;
    let event_nft_data = pt.context.banks_client.get_account(event_nft).await.unwrap().unwrap();
    let event_nft_data = TokenMint::try_deserialize_unchecked(&mut &event_nft_data.data[..]).unwrap();

    // mint authority is transferred to the master edition when the latter is created
    assert_eq!(event_nft_data.mint_authority.unwrap(), find_master_edition_account(&event_nft).0);
    assert_eq!(event_nft_data.supply, 1);
  }

  // Assert the ATA account
  {
    let mut pt = runner.pt.lock().await;

    let event_organizer_ata = pda::event_organizer_ata(&event_organizer.pubkey(), &event_nft);
    let event_organizer_ata = pt.context.banks_client.get_account(event_organizer_ata).await.unwrap().unwrap();
    let event_organizer_ata = TokenAccount::try_deserialize_unchecked(&mut &event_organizer_ata.data[..]).unwrap();
  
    assert_eq!(event_organizer_ata.mint, event_nft);
    assert_eq!(event_organizer_ata.owner, event_organizer.pubkey());
    assert_eq!(event_organizer_ata.amount, 1);
  }
}

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_fail_if_event_capacity_is_not_owned_by_the_ticket_sale_program(ctx: &mut TestContext) {
  let runner = &mut ctx.event_registry_runner;
  let ticket_sale_runner = &mut ctx.ticket_sale_runner;
  let state = Keypair::new();
  let event_id = 0;
  let event_organizer = runner.get_participant(1);
  
  let event_capacity;
  {
    let mut pt_lock = runner.pt.lock().await;
    let space = 8 + std::mem::size_of::<EventCapacity>() + event_capacity_space_margin + (10_000 / 8) as usize + 8;
    event_capacity = pt_lock.create_account(
      sol_to_lamports(1000_f64),
      space as u64, 
      &system_program::ID, // the owner must be the ticket sale program
    ).await.pubkey();
  }

  let result = custom_create_event(
    false,
    runner,
    ticket_sale_runner,
    &state,
    event_capacity,
    event_id,
    &event_organizer,
    0,
  ).await;

  Error::assert_err(result, event_registry::utils::program_error::ErrorCode::TicketSaleMustBeOwner);
}

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_fail_if_max_ticket_types_violated(ctx: &mut TestContext) {
  let state = Keypair::new();
  let runner = &mut ctx.event_registry_runner;
  let ticket_sale_runner = &mut ctx.ticket_sale_runner;
  let event_capacity = runner.create_event_capacity_account().await;
  
  runner.initialize(
    &state,
		1_000, // 10%
  ).await;

  let event_id = 0;
  let event_organizer = runner.get_participant(1);
  let deposit_token = runner.deposit_tokens[0];

  // Create more that 10 ticket types which is the current limit
  let mut ticket_types = vec![];

  for _ in 0..11 {
    ticket_types.push(
      TicketType {
        n_tickets: 1000,
        sale_type: SaleType::FixedPrice(to_base(100, 6)),
        sale_start_time: 50,
        merkle_root: [0; 32],
      }
    )
  }

  let ticket_sale_program_state = initialize_ticket_sale(ticket_sale_runner, state.pubkey()).await;

  let result = runner.create_event(
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
		"Ticket Land Coolest Event".to_owned(),
		"TICKT".to_owned(),
		"https://ticketland.io".to_owned(),
  ).await;

  Error::assert_err(result, event_registry::utils::program_error::ErrorCode::TooManyTicketTypes);
}

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_transfer_deposit_amount_to_fund_manager_ata(ctx: &mut TestContext) {
  let runner = &mut ctx.event_registry_runner;
  let ticket_sale_runner = &mut ctx.ticket_sale_runner;
  let state = Keypair::new();
  let event_capacity = runner.create_event_capacity_account().await;
  let event_id = 0;
  let event_organizer = runner.get_participant(1);
  
  let _ = custom_create_event(
    false,
    runner,
    ticket_sale_runner,
    &state,
    event_capacity,
    event_id,
    &event_organizer,
    0,
  ).await;

  let deposit_token = runner.deposit_tokens[0];
  let event = pda::event(&state.pubkey(), event_id).0;
  let fund_manager = pda::fund_manager(&state.pubkey(), &event, &event_organizer.pubkey()).0;
  let fund_manager_ata = pda::fund_manager_ata(&fund_manager, &deposit_token);

  assert_eq!(runner.spl.get_token_account(fund_manager_ata).await.amount, to_base(1000, 6));
}

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_accept_native_sol_as_deposit(ctx: &mut TestContext) {
  let runner = &mut ctx.event_registry_runner;
  let ticket_sale_runner = &mut ctx.ticket_sale_runner;
  let state = Keypair::new();
  let event_capacity = runner.create_event_capacity_account().await;
  let event_id = 0;
  let event_organizer = runner.get_participant(1);

  let _ = custom_create_event(
    false,
    runner,
    ticket_sale_runner,
    &state,
    event_capacity,
    event_id,
    &event_organizer,
    2, // native sol
  ).await;

  {
    let event = pda::event(&state.pubkey(), event_id).0;
    let fund_manager = pda::fund_manager(&state.pubkey(), &event, &event_organizer.pubkey()).0;
    let mut pt = runner.pt.lock().await;
    let account = pt.context.banks_client.get_account(fund_manager).await.unwrap().unwrap();

    // Not 890880 is the lamports stored in the account balance because of rent exception
    // when the account was create
    assert_eq!(account.lamports, sol_to_lamports(10_f64) + 890880);
  }
}

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_not_allow_user_control_fund_manager_ata(ctx: &mut TestContext) {
  let runner = &mut ctx.event_registry_runner;
  let state = Keypair::new();
  let ticket_sale_runner = &mut ctx.ticket_sale_runner;
  let event_capacity = runner.create_event_capacity_account().await;
  let event_id = 0;
  let event_organizer = runner.get_participant(1);
  
  let _ = custom_create_event(
    false,
    runner,
    ticket_sale_runner,
    &state,
    event_capacity,
    event_id,
    &event_organizer,
    0,
  ).await;

  let deposit_token = runner.deposit_tokens[0];
  let event = pda::event(&state.pubkey(), event_id).0;
  let fund_manager = pda::fund_manager(&state.pubkey(), &event, &event_organizer.pubkey()).0;
  let fund_manager_ata = pda::fund_manager_ata(&fund_manager, &deposit_token);
  let event_organizer_ata = pda::event_organizer_ata(&event_organizer.pubkey(), &deposit_token);

  // Organizer tries to transfer the funds from the fund manager ata
  let result = runner.spl.transfer(
    &fund_manager_ata, 
    &event_organizer_ata,
    &event_organizer,
    to_base(1000, 6),
  ).await;

  assert!(!result.is_ok());
}

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_fail_if_event_organizer_has_not_enough_balance_to_deposit(ctx: &mut TestContext) {
  let runner = &mut ctx.event_registry_runner;
  let ticket_sale_runner = &mut ctx.ticket_sale_runner;
  let state = Keypair::new();
  let event_capacity = runner.create_event_capacity_account().await;
  let event_id = 0;
  let event_organizer = Keypair::new();
  let event_organizer_clone = Keypair::from_bytes(event_organizer.to_bytes().as_ref()).unwrap();

  runner.initialize(
    &state,
		1_000, // 10%
  ).await;

  runner.spl.airdrop(
    &runner.deposit_tokens[0],
    &runner.deposit_token_authorities[0],
    &vec![event_organizer],
    to_base(999, 6), // 1 less than the min deposit amount
  ).await;

  let result = custom_create_event(
    true,
    runner,
    ticket_sale_runner,
    &state,
    event_capacity,
    event_id,
    &event_organizer_clone,
    0,
  ).await;

  assert!(!result.is_ok());
}

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_fail_if_deposit_token_not_supported(ctx: &mut TestContext) {
  let runner = &mut ctx.event_registry_runner;
  let ticket_sale_runner = &mut ctx.ticket_sale_runner;
  let state = Keypair::new();
  let event_capacity = runner.create_event_capacity_account().await;
  let event_id = 0;
  let event_organizer = runner.get_participant(1);
  let event_organizer_clone = Keypair::from_bytes(event_organizer.to_bytes().as_ref()).unwrap();

  // create a new token that is not part of the supported currencies
  let mint_token = Keypair::new();
  let authority = Keypair::new();

  runner.spl.create_mint(
    &mint_token,
    &authority.pubkey(),
    None,
    6
  ).await;

  runner.spl.airdrop(
    &mint_token.pubkey(),
    &authority,
    &vec![event_organizer],
    to_base(1_000_000, 6),
  ).await;

  let ticket_types = vec![
    TicketType {
      n_tickets: 1000,
      sale_type: SaleType::FixedPrice(to_base(100, 6)),
      sale_start_time: 50,
      merkle_root: [0; 32],
    },
    TicketType {
      n_tickets: 1000,
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

  runner.initialize(
    &state,
    1_000, // 10%
  ).await;

  let ticket_sale_program_state = initialize_ticket_sale(ticket_sale_runner, state.pubkey()).await;
  let result = runner.create_event(
    state.pubkey(),
    event_capacity,
    ticket_sale_program_state,
    event_id,
    mint_token.pubkey(),
    mint_token.pubkey(),
    &event_organizer_clone,
    100_000,
    100,
		1000,
		ticket_types.clone(),
		"Ticket Land Coolest Event".to_owned(),
		"TICKT".to_owned(),
		"https://ticketland.io".to_owned(),
  ).await;

  Error::assert_err(result, event_registry::utils::program_error::ErrorCode::UnsupportedDepositToken);
}
