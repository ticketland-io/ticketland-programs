#![cfg(feature = "test-bpf")]
use test_context::{test_context, futures};
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
      ticket_nft_state.pubkey(),
      purchase_token,
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
async fn should_enforce_access_control(ctx: &mut TestContext) {
  let (
    event_registry_state,
    ticket_sale_state,
    ticket_nft_state,
    secondary_market_state,
    event_id,
    seat_index,
    event_organizer,
    ticket_owner, // ticket buyer from primary market is the ticket owner
    purchase_token,
    ticket_type_index,
    ticket_types,
  ) = before_each(ctx).await;

  let sale = TicketSalePda::ticket_sale_state(
    &ticket_sale_state.pubkey(),
    ticket_type_index,
    event_id,
  ).0;

  // should fail if wrong sale account is passed
  {
    let some_other_event_id: [u8; 32] = "aaaa6394e04a4b3c8ccd7e2772cb14b4".to_owned().into_bytes().try_into().unwrap();
    
    {
      // prepare new accounts
      let event_registry_runner = &mut ctx.event_registry_runner;
      let deposit_token = event_registry_runner.deposit_tokens[2];
      // create a new event
      let _ = setup(
        ctx,
        &event_organizer,
        &ticket_owner,
        event_registry_state.pubkey(),
        ticket_sale_state.pubkey(),
        ticket_nft_state.pubkey(),
        deposit_token,
        purchase_token,
        some_other_event_id,
        seat_index,
        ticket_type_index,
      ).await;

      let secondary_market_runner = &mut ctx.secondary_market_runner;

      // create a market for that sale
      let result = secondary_market_runner.create_market(
        secondary_market_state.pubkey(),
        event_registry_state.pubkey(),
        some_other_event_id,
        &event_organizer,
        500, // organizer_resale_fee 5%
        1000, // resale_cap 10%
      ).await;
      assert!(result.is_ok());
    }

    // now use this new sale to create listing for a ticket that was purchased in the previous sale
    let sale = TicketSalePda::ticket_sale_state(
      &ticket_sale_state.pubkey(),
      ticket_type_index,
      some_other_event_id,
    ).0;

    let event_registry_runner = &mut ctx.event_registry_runner;
    let secondary_market_runner = &mut ctx.secondary_market_runner;
    let treasury = event_registry_runner.get_participant(5);
    let ticket_buyer = event_registry_runner.get_participant(4);
    
    let result = secondary_market_runner.fill_sell_listing(
      event_id,
      secondary_market_state.pubkey(),
      event_registry_state.pubkey(),
      sale,
      seat_index,
      ticket_nft_state.pubkey(),
      purchase_token,
      treasury.pubkey(),
      ticket_owner.pubkey(),
      &ticket_buyer,
      event_organizer.pubkey(),
    ).await;

    Error::assert_err(result, secondary_market::utils::program_error::ErrorCode::WrongSaleAccount);
  }

  // should fail if wrong purchase token is given
  {
    let secondary_market_runner = &mut ctx.secondary_market_runner;
    let event_registry_runner = &mut ctx.event_registry_runner;
    let wrong_purchase_token = event_registry_runner.deposit_tokens[1];
    let treasury = event_registry_runner.get_participant(5);
    let ticket_buyer = event_registry_runner.get_participant(4);

    let result = secondary_market_runner.fill_sell_listing(
      event_id,
      secondary_market_state.pubkey(),
      event_registry_state.pubkey(),
      sale,
      seat_index,
      ticket_nft_state.pubkey(),
      wrong_purchase_token,
      treasury.pubkey(),
      ticket_owner.pubkey(),
      &ticket_buyer,
      event_organizer.pubkey(),
    ).await;

    Error::assert_err(result, secondary_market::utils::program_error::ErrorCode::WrongPurchaseToken);
  }

  // should fail if wrong event_organizer account is used
  {
    let secondary_market_runner = &mut ctx.secondary_market_runner;
    let event_registry_runner = &mut ctx.event_registry_runner;
    let treasury = event_registry_runner.get_participant(5);
    let ticket_buyer = event_registry_runner.get_participant(4);
    let wrong_event_organizer = event_registry_runner.get_participant(2);
    
    let result = secondary_market_runner.fill_sell_listing(
      event_id,
      secondary_market_state.pubkey(),
      event_registry_state.pubkey(),
      sale,
      seat_index,
      ticket_nft_state.pubkey(),
      purchase_token,
      treasury.pubkey(),
      ticket_owner.pubkey(),
      &ticket_buyer,
      wrong_event_organizer.pubkey(),
    ).await;

    Error::assert_err(result, secondary_market::utils::program_error::ErrorCode::WrongEventOrganizer);
  }

  // move to the end of sale
  {
    let ticket_sale_runner = &mut ctx.ticket_sale_runner;
    let mut pt = ticket_sale_runner.pt.lock().await;
    pt.advance_clock_past_timestamp(ticket_types[0].sale_end_time + 1).await;
  }
  
  // Should fail if ticket sale is finished
  {
    let secondary_market_runner = &mut ctx.secondary_market_runner;
    let event_registry_runner = &mut ctx.event_registry_runner;
    let treasury = event_registry_runner.get_participant(5);
    let ticket_buyer = event_registry_runner.get_participant(4);
    
    let result = secondary_market_runner.fill_sell_listing(
      event_id,
      secondary_market_state.pubkey(),
      event_registry_state.pubkey(),
      sale,
      seat_index,
      ticket_nft_state.pubkey(),
      purchase_token,
      treasury.pubkey(),
      ticket_owner.pubkey(),
      &ticket_buyer,
      event_organizer.pubkey(),
    ).await;
    
    Error::assert_ticket_sale_err(result, ticket_sale::utils::program_error::ErrorCode::SaleFinished);
  }
}

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_fail_when_wrong_ticket_owner(ctx: &mut TestContext) {
  let (
    event_registry_state,
    ticket_sale_state,
    ticket_nft_state,
    secondary_market_state,
    event_id,
    seat_index,
    event_organizer,
    _,
    purchase_token,
    ticket_type_index,
    _,
  ) = before_each(ctx).await;

  let sale = TicketSalePda::ticket_sale_state(
    &ticket_sale_state.pubkey(),
    ticket_type_index,
    event_id,
  ).0;

  let secondary_market_runner = &mut ctx.secondary_market_runner;
  let event_registry_runner = &mut ctx.event_registry_runner;
  let treasury = event_registry_runner.get_participant(5);
  let ticket_buyer = event_registry_runner.get_participant(4);
  let wrong_ticket_owner = event_registry_runner.get_participant(1);
  
  let result = secondary_market_runner.fill_sell_listing(
    event_id,
    secondary_market_state.pubkey(),
    event_registry_state.pubkey(),
    sale,
    seat_index,
    ticket_nft_state.pubkey(),
    purchase_token,
    treasury.pubkey(),
    wrong_ticket_owner.pubkey(),
    &ticket_buyer,
    event_organizer.pubkey(),
  ).await;

  Error::assert_err(result, secondary_market::utils::program_error::ErrorCode::WrongTicketSeller);
}

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_fail_when_wrong_treasury(ctx: &mut TestContext) {
  let (
    event_registry_state,
    ticket_sale_state,
    ticket_nft_state,
    secondary_market_state,
    event_id,
    seat_index,
    event_organizer,
    ticket_owner, // ticket buyer from primary market is the ticket owner
    purchase_token,
    ticket_type_index,
    _,
  ) = before_each(ctx).await;

  let sale = TicketSalePda::ticket_sale_state(
    &ticket_sale_state.pubkey(),
    ticket_type_index,
    event_id,
  ).0;

  let secondary_market_runner = &mut ctx.secondary_market_runner;
  let event_registry_runner = &mut ctx.event_registry_runner;
  let ticket_buyer = event_registry_runner.get_participant(4);
  let wrong_treasury = event_registry_runner.get_participant(1);
  
  let result = secondary_market_runner.fill_sell_listing(
    event_id,
    secondary_market_state.pubkey(),
    event_registry_state.pubkey(),
    sale,
    seat_index,
    ticket_nft_state.pubkey(),
    purchase_token,
    wrong_treasury.pubkey(),
    ticket_owner.pubkey(),
    &ticket_buyer,
    event_organizer.pubkey(),
  ).await;

  Error::assert_err(result, secondary_market::utils::program_error::ErrorCode::WrongTreasuryAccount);
}

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_transfer_funds(ctx: &mut TestContext) {
  let (
    event_registry_state,
    ticket_sale_state,
    ticket_nft_state,
    secondary_market_state,
    event_id,
    seat_index,
    event_organizer,
    ticket_owner, // ticket buyer from primary market is the ticket owner
    purchase_token,
    ticket_type_index,
    _,
  ) = before_each(ctx).await;

  let secondary_market_runner = &mut ctx.secondary_market_runner;
  let sale = TicketSalePda::ticket_sale_state(
    &ticket_sale_state.pubkey(),
    ticket_type_index,
    event_id,
  ).0;

  let event_registry_runner = &mut ctx.event_registry_runner;
  let treasury = event_registry_runner.get_participant(5);
  let ticket_buyer = event_registry_runner.get_participant(4);
  
  let (
    treasury_balance_before,
    event_organizer_balance_before,
    ticket_owner_balance_before,
  ) = secondary_market_runner.get_ata_balances(treasury.pubkey(), event_organizer.pubkey(), ticket_owner.pubkey(), purchase_token).await;

  let result = secondary_market_runner.fill_sell_listing(
    event_id,
    secondary_market_state.pubkey(),
    event_registry_state.pubkey(),
    sale,
    seat_index,
    ticket_nft_state.pubkey(),
    purchase_token,
    treasury.pubkey(),
    ticket_owner.pubkey(),
    &ticket_buyer,
    event_organizer.pubkey(),
  ).await;
  assert!(result.is_ok());

  let (
    treasury_balance_after,
    event_organizer_balance_after,
    ticket_owner_balance_after,
  ) = secondary_market_runner.get_ata_balances(treasury.pubkey(), event_organizer.pubkey(), ticket_owner.pubkey(), purchase_token).await;

  // 5% goes to treasury and 5% to the event organizer
  // The sell price is 1.1
  assert_eq!(treasury_balance_after - treasury_balance_before, sol_to_lamports(0.055_f64));
  assert_eq!(event_organizer_balance_after - event_organizer_balance_before, sol_to_lamports(0.055_f64));
  assert_eq!(ticket_owner_balance_after - ticket_owner_balance_before, sol_to_lamports(0.99_f64));
}

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_change_ownership_of_the_ticket(ctx: &mut TestContext) {
  let (
    event_registry_state,
    ticket_sale_state,
    ticket_nft_state,
    secondary_market_state,
    event_id,
    seat_index,
    event_organizer,
    ticket_owner, // ticket buyer from primary market is the ticket owner
    purchase_token,
    ticket_type_index,
    _,
  ) = before_each(ctx).await;


  let secondary_market_runner = &mut ctx.secondary_market_runner;
  let sale = TicketSalePda::ticket_sale_state(
    &ticket_sale_state.pubkey(),
    ticket_type_index,
    event_id,
  ).0;

  let event_registry_runner = &mut ctx.event_registry_runner;
  let treasury = event_registry_runner.get_participant(5);
  let ticket_buyer = event_registry_runner.get_participant(4);

  let result = secondary_market_runner.fill_sell_listing(
    event_id,
    secondary_market_state.pubkey(),
    event_registry_state.pubkey(),
    sale,
    seat_index,
    ticket_nft_state.pubkey(),
    purchase_token,
    treasury.pubkey(),
    ticket_owner.pubkey(),
    &ticket_buyer,
    event_organizer.pubkey(),
  ).await;
  assert!(result.is_ok());

  {
    let ticket_nft = TicketNftPda::ticket_nft(&ticket_nft_state.pubkey(), seat_index, event_id).0;
    let ticket_metadata = TicketNftPda::ticket_metadata(&ticket_nft_state.pubkey(), &ticket_nft).0;

    let mut pt = secondary_market_runner.pt.lock().await;
    let ticket_metadata_data = pt.get_account::<ticket_nft::account_data::ticket_metadata::TicketMetadata>(ticket_metadata).await;

    assert_eq!(ticket_metadata_data.owner, ticket_buyer.pubkey());
  }
}

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_close_sell_listing_account(ctx: &mut TestContext) {
  let (
    event_registry_state,
    ticket_sale_state,
    ticket_nft_state,
    secondary_market_state,
    event_id,
    seat_index,
    event_organizer,
    ticket_owner, // ticket buyer from primary market is the ticket owner
    purchase_token,
    ticket_type_index,
    _,
  ) = before_each(ctx).await;


  let secondary_market_runner = &mut ctx.secondary_market_runner;
  let sale = TicketSalePda::ticket_sale_state(
    &ticket_sale_state.pubkey(),
    ticket_type_index,
    event_id,
  ).0;

  let event_registry_runner = &mut ctx.event_registry_runner;
  let treasury = event_registry_runner.get_participant(5);
  let ticket_buyer = event_registry_runner.get_participant(4);

  let result = secondary_market_runner.fill_sell_listing(
    event_id,
    secondary_market_state.pubkey(),
    event_registry_state.pubkey(),
    sale,
    seat_index,
    ticket_nft_state.pubkey(),
    purchase_token,
    treasury.pubkey(),
    ticket_owner.pubkey(),
    &ticket_buyer,
    event_organizer.pubkey(),
  ).await;
  assert!(result.is_ok());

  {
    let ticket_nft = TicketNftPda::ticket_nft(&ticket_nft_state.pubkey(), seat_index, event_id).0;
    let ticket_metadata = TicketNftPda::ticket_metadata(&ticket_nft_state.pubkey(), &ticket_nft).0;
    let sell_listing = pda::sell_listing(&secondary_market_state.pubkey(), event_id, &ticket_metadata).0;

    let mut pt = secondary_market_runner.pt.lock().await;
    let account = pt.context.banks_client.get_account(sell_listing).await.unwrap();

    assert!(account.is_none());
  }
}

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_allow_new_owner_list_ticket_for_sale(ctx: &mut TestContext) {
  let (
    event_registry_state,
    ticket_sale_state,
    ticket_nft_state,
    secondary_market_state,
    event_id,
    seat_index,
    event_organizer,
    ticket_owner, // ticket buyer from primary market is the ticket owner
    purchase_token,
    ticket_type_index,
    _,
  ) = before_each(ctx).await;

  let secondary_market_runner = &mut ctx.secondary_market_runner;
  let sale = TicketSalePda::ticket_sale_state(
    &ticket_sale_state.pubkey(),
    ticket_type_index,
    event_id,
  ).0;

  let event_registry_runner = &mut ctx.event_registry_runner;
  let treasury = event_registry_runner.get_participant(5);
  let ticket_buyer = event_registry_runner.get_participant(4);

  let result = secondary_market_runner.fill_sell_listing(
    event_id,
    secondary_market_state.pubkey(),
    event_registry_state.pubkey(),
    sale,
    seat_index,
    ticket_nft_state.pubkey(),
    purchase_token,
    treasury.pubkey(),
    ticket_owner.pubkey(),
    &ticket_buyer,
    event_organizer.pubkey(),
  ).await;
  assert!(result.is_ok());

  // The new owner can list the ticket for sale again
  {
    let secondary_market_runner = &mut ctx.secondary_market_runner;
    let sale = TicketSalePda::ticket_sale_state(
      &ticket_sale_state.pubkey(),
      ticket_type_index,
      event_id,
    ).0;

    let result = secondary_market_runner.create_sell_listing(
      event_id,
      // 5% higher than the price sold in the primary market.
      // cap is at 10%
      sol_to_lamports(1.05),
      secondary_market_state.pubkey(),
      event_registry_state.pubkey(),
      sale,
      seat_index,
      ticket_nft_state.pubkey(),
      purchase_token,
      &ticket_buyer, // the new owner
    ).await;
    assert!(result.is_ok());
  }
}
