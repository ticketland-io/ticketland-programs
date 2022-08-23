#![cfg(feature = "test-bpf")]
use test_context::{test_context, futures};
use solana_sdk::{
  signature::{Signer},
  native_token::sol_to_lamports,
};
use solana_program_test::{tokio};
use common_test::{
  test_context::TestContext,
  ticket_sale::{
    pda::{self as TicketSalePda},
  },
  secondary_market::{
    common::{init, setup},
    error::Error,
  }
};

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_enforce_access_control(ctx: &mut TestContext) {
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
  
  // should fail if wrong sale account is passed
  {
    let some_other_event_id: [u8; 32] = "aaaa6394e04a4b3c8ccd7e2772cb14b4".to_owned().into_bytes().try_into().unwrap();
    
    {
      // create a new event
      let _ = setup(
        ctx,
        &event_organizer,
        &ticket_buyer,
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
  
    let secondary_market_runner = &mut ctx.secondary_market_runner;
    let result = secondary_market_runner.create_sell_listing(
      event_id,
      // 4% higher than the price sold in the primary market.
      // cap is at 5%
      sol_to_lamports(1.04),
      secondary_market_state.pubkey(),
      event_registry_state.pubkey(),
      sale,
      seat_index,
      ticket_nft_state.pubkey(),
      purchase_token,
      &ticket_buyer,
    ).await;
  
    Error::assert_err(result, secondary_market::utils::program_error::ErrorCode::WrongSaleAccount);
  }

  // Should fail is asking price higher than the price cap
  {
    let secondary_market_runner = &mut ctx.secondary_market_runner;

    let sale = TicketSalePda::ticket_sale_state(
      &ticket_sale_state.pubkey(),
      ticket_type_index,
      event_id,
    ).0;

    let result = secondary_market_runner.create_sell_listing(
      event_id,
      // 5.1% higher than the price sold in the primary market.
      // cap is at 5%
      sol_to_lamports(1.051),
      secondary_market_state.pubkey(),
      event_registry_state.pubkey(),
      sale,
      seat_index,
      ticket_nft_state.pubkey(),
      purchase_token,
      &ticket_buyer,
    ).await;

    Error::assert_err(result, secondary_market::utils::program_error::ErrorCode::PriceCap);
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

    let sale = TicketSalePda::ticket_sale_state(
      &ticket_sale_state.pubkey(),
      ticket_type_index,
      event_id,
    ).0;

    let result = secondary_market_runner.create_sell_listing(
      event_id,
      // 4% higher than the price sold in the primary market.
      // cap is at 5%
      sol_to_lamports(1.04),
      secondary_market_state.pubkey(),
      event_registry_state.pubkey(),
      sale,
      seat_index,
      ticket_nft_state.pubkey(),
      purchase_token,
      &ticket_buyer,
    ).await;

    Error::assert_ticket_sale_err(result, ticket_sale::utils::program_error::ErrorCode::SaleFinished);
  }
}
