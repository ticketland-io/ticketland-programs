#![cfg(feature = "test-bpf")]
mod utils;

use std::{assert_eq};
use anchor_lang::prelude::Pubkey;
use test_context::{test_context, futures};
use solana_sdk::{
  signature::{Signer, Keypair},
};
use solana_test_utils::{
  utils::{to_base},
  spl::Spl,
};
use anchor_metaplex::{
  mpl_token_metadata::{
    deser::meta_deser,
    pda::{find_metadata_account},
  },
};
use solana_program_test::{tokio};
use utils::{
  pda,
  test_context::TestContext,
};
use common::{
  state::{
    ticket_type::TicketType,
    sale_type::SaleType,
  },
};

#[test_context(TestContext)]
#[tokio::test(flavor = "multi_thread")]
async fn should_create_a_new_event(ctx: &mut TestContext) {
  let state = Keypair::new();
  let runner = &mut ctx.runner;
  
  runner.initialize(
    &state,
    500, // 5%
		1_000, // 10%
  ).await;

  let event_id = 0;
  let event_organizer = runner.get_participant(1);
  let deposit_token = runner.deposit_tokens[0];
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

  let (fund_manager, _) = pda::fund_manager(&state.pubkey(), &event_organizer.pubkey());
  let fund_manager_ata = pda::fund_manager_ata(&fund_manager, &deposit_token);
  runner.add_create_event_deposit(
    deposit_token,
    to_base(1000, 6),
    &event_organizer,
    fund_manager_ata,
  ).await;

  let result = runner.create_event(
    state.pubkey(),
    event_id,
    deposit_token,
    &event_organizer,
    100,
		1000,
		ticket_types.clone(),
		"Ticket Land Coolest Event".to_owned(),
		"TICKT".to_owned(),
		"https://ticketland.io".to_owned(),
  ).await;

  assert!(result.is_ok());

  {
    let event_nft = pda::event_nft(&state.pubkey(), event_id).0;
    let metadata = find_metadata_account(&event_nft).0;
    let mut pt = runner.pt.lock().await;
    let account = pt.context.banks_client.get_account(metadata).await.unwrap().unwrap();
    let metadata = meta_deser(&mut &account.data[..]).unwrap();

    println!(">>>>> {:?}", metadata);
  }

}
