#![cfg(feature = "test-bpf")]
use test_context::{test_context};
use solana_sdk::{
  signature::{Signer, Keypair},
  pubkey::Pubkey,
  native_token::sol_to_lamports,
};
use solana_program_test::{tokio};
use solana_test_utils::{
  spl::Spl,
};
use common::{
  state::{
    ticket_type::{TicketType},
  },
};
use common_test::{
  test_context::TestContext,
  secondary_market::{
    common::{init, setup},
    error::Error,
    pda
  }
};

async fn before_each(ctx: &mut TestContext, bid_price: u64) -> (
  Keypair,
  Keypair,
  Keypair,
  Keypair,
  Keypair,
  Keypair,
  Keypair,
  Pubkey,
  [u8; 32],
  u8,
  u16,
  u32,
  Vec<TicketType>
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
  let ticket_owner = event_registry_runner.get_participant(3);
  let deposit_token = event_registry_runner.deposit_tokens[2];
  let purchase_token = deposit_token;
  let ticket_type_index = 0;
  let n_listing = 0;

  let (ticket_types,) = setup(
    ctx,
    &event_organizer,
    &ticket_owner,
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
    let result = secondary_market_runner.create_buy_listing(
      event_id,
      bid_price,
      secondary_market_state.pubkey(),
      event_registry_state.pubkey(),
      purchase_token,
      &ticket_buyer,
      n_listing,
    ).await;
    assert!(result.is_ok());
  }

  (
    event_registry_state,
    secondary_market_state,
    ticket_sale_state,
    ticket_nft_state,
    ticket_buyer,
    ticket_owner,
    event_organizer,
    purchase_token,
    event_id,
    ticket_type_index,
    n_listing,
    seat_index,
    ticket_types,
  )
}

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
#[ignore = "passing"]
async fn should_fail_if_wrong_purchase_token(ctx: &mut TestContext) {
  let (
    event_registry_state,
    secondary_market_state,
    _,
    _,
    ticket_buyer,
    _,
    _,
    _,
    event_id,
    _,
    n_listing,
    _,
    _,
  ) = before_each(ctx, sol_to_lamports(1.1)).await;

  let event_registry_runner = &mut ctx.event_registry_runner;
  let wrong_purchase_token = event_registry_runner.deposit_tokens[1];

  // create the listing ATA so it doesn't fail due to an inexistent account
  {
    let buy_listing = pda::buy_listing(&secondary_market_state.pubkey(), event_id, &ticket_buyer.pubkey(), n_listing).0;
    let listing_escrow = pda::listing_escrow(&secondary_market_state.pubkey(), event_id, &buy_listing).0;

    let secondary_market_runner = &mut ctx.secondary_market_runner;
    secondary_market_runner.spl.create_associated_account(&listing_escrow, &wrong_purchase_token).await;
  }

  let secondary_market_runner = &mut ctx.secondary_market_runner;

  let result = secondary_market_runner.cancel_buy_listing(
    event_id,
    secondary_market_state.pubkey(),
    event_registry_state.pubkey(),
    wrong_purchase_token,
    &ticket_buyer,
    n_listing,
  ).await;

  Error::assert_err(result, secondary_market::utils::program_error::ErrorCode::WrongPurchaseToken);
}

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
#[ignore = "passing"]
async fn should_close_the_listing_and_escrow_ata_accounts(ctx: &mut TestContext) {
  let (
    event_registry_state,
    secondary_market_state,
    _,
    _,
    ticket_buyer,
    _,
    _,
    purchase_token,
    event_id,
    _,
    n_listing,
    _,
    _,
  ) = before_each(ctx, sol_to_lamports(1.1)).await;

  let secondary_market_runner = &mut ctx.secondary_market_runner;

  let result = secondary_market_runner.cancel_buy_listing(
    event_id,
    secondary_market_state.pubkey(),
    event_registry_state.pubkey(),
    purchase_token,
    &ticket_buyer,
    n_listing,
  ).await;
  assert!(result.is_ok());

  {
    let buy_listing = pda::buy_listing(&secondary_market_state.pubkey(), event_id, &ticket_buyer.pubkey(), n_listing).0;
    let mut pt = secondary_market_runner.pt.lock().await;
    let account = pt.context.banks_client.get_account(buy_listing).await.unwrap();
    assert!(account.is_none());

    let listing_escrow = pda::listing_escrow(&secondary_market_state.pubkey(), event_id, &buy_listing).0;
    let listing_escrow_ata = Spl::get_associated_token_address(&listing_escrow, &purchase_token);
    let account = pt.context.banks_client.get_account(listing_escrow_ata).await.unwrap();
    assert!(account.is_none());
  }
}
