#![cfg(feature = "test-bpf")]
use test_context::{test_context};
use solana_sdk::{
  signature::{Signer, Keypair},
  pubkey::Pubkey,
  native_token::sol_to_lamports,
};
use solana_program_test::{tokio};
use common::{
  state::{
    ticket_type::{TicketType},
  },
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

async fn before_each(ctx: &mut TestContext) -> (
  Keypair,
  Keypair,
  Keypair,
  Keypair,
  [u8; 32],
  u32,
  Keypair,
  Keypair,
  Pubkey,
  u8,
  Vec<TicketType>,
) {
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
  let ticket_type_index = 0;

  let (ticket_types,) = setup(
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
    ticket_type_index,
  ).await;

  {
    let secondary_market_runner = &mut ctx.secondary_market_runner;
    
    let result = secondary_market_runner.create_market(
      secondary_market_state.pubkey(),
      event_registry_state.pubkey(),
      event_id,
      &event_organizer,
      500, // organizer_resale_fee 5%
      1000, // resale_cap 10%
    ).await;
    assert!(result.is_ok());
  }
  
  {
    let secondary_market_runner = &mut ctx.secondary_market_runner;
    let sale = TicketSalePda::ticket_sale_state(
      &ticket_sale_state.pubkey(),
      ticket_type_index,
      event_id,
    ).0;

    let result = secondary_market_runner.create_sell_listing(
      event_id,
      // 10% higher than the price sold in the primary market.
      // cap is at 10%
      sol_to_lamports(1.1),
      secondary_market_state.pubkey(),
      event_registry_state.pubkey(),
      sale,
      seat_index,
      ticket_type_index,
      ticket_nft_state.pubkey(),
      &ticket_buyer, 
    ).await;
    assert!(result.is_ok());
  }

  (
    event_registry_state,
    ticket_sale_state,
    ticket_nft_state,
    secondary_market_state,
    event_id,
    seat_index,
    event_organizer,
    ticket_buyer,
    purchase_token,
    ticket_type_index,
    ticket_types,
  )
}

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_fail_when_wrong_ticket_owner(ctx: &mut TestContext) {
  let (
    event_registry_state,
    _,
    ticket_nft_state,
    secondary_market_state,
    event_id,
    seat_index,
    _,
    _,
    _,
    ticket_type_index,
    _,
  ) = before_each(ctx).await;

  let secondary_market_runner = &mut ctx.secondary_market_runner;
  let event_registry_runner = &mut ctx.event_registry_runner;
  let wrong_ticket_owner = event_registry_runner.get_participant(1);
  
  let result = secondary_market_runner.cancel_sell_listing(
    event_id,
    secondary_market_state.pubkey(),
    event_registry_state.pubkey(),
    seat_index,
    ticket_type_index,
    ticket_nft_state.pubkey(),
    &wrong_ticket_owner,
  ).await;

  Error::assert_err(result, secondary_market::utils::program_error::ErrorCode::OnlyTicketOwner);
}

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_close_sell_listing_account(ctx: &mut TestContext) {
  let (
    event_registry_state,
    _,
    ticket_nft_state,
    secondary_market_state,
    event_id,
    seat_index,
    _,
    ticket_owner, // ticket buyer from primary market is the ticket owner
    _,
    ticket_type_index,
    _,
  ) = before_each(ctx).await;


  let secondary_market_runner = &mut ctx.secondary_market_runner;

  let result = secondary_market_runner.cancel_sell_listing(
    event_id,
    secondary_market_state.pubkey(),
    event_registry_state.pubkey(),
    seat_index,
    ticket_type_index,
    ticket_nft_state.pubkey(),
    &ticket_owner,
  ).await;
  assert!(result.is_ok());

  {
    let ticket_nft = TicketNftPda::ticket_nft(&ticket_nft_state.pubkey(), seat_index, event_id, ticket_type_index).0;
    let ticket_metadata = TicketNftPda::ticket_metadata(&ticket_nft_state.pubkey(), &ticket_nft).0;
    let sell_listing = pda::sell_listing(&secondary_market_state.pubkey(), event_id, &ticket_metadata).0;

    let mut pt = secondary_market_runner.pt.lock().await;
    let account = pt.context.banks_client.get_account(sell_listing).await.unwrap();

    assert!(account.is_none());
  }
}
