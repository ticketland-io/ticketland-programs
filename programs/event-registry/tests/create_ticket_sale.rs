#![cfg(feature = "test-bpf")]

use test_context::{test_context, futures};
use anchor_lang::{prelude::*};
use solana_sdk::{
  signature::{Signer, Keypair},
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

async fn custom_create__event(
  skip_init: bool,
  runner: &mut Runner,
  state: &Keypair,
  event_id: u64,
  event_organizer: &Keypair,
  deposit_token_idx: usize,
) -> AnchorResult<()> {
  if !skip_init {
    runner.initialize(
      &state,
      500, // 5%
      1_000, // 10%
    ).await;
  }

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
    event_id,
    deposit_token,
    &event_organizer,
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
async fn should_create_a_new_sale_by_calling_the_ticket_sale_program(ctx: &mut TestContext) {
  let runner = &mut ctx.event_registry_runner;
  let state = Keypair::new();
  let event_id = 0;
  let event_organizer = runner.get_participant(1);
  
  let result = custom_create__event(
    false,
    runner,
    &state,
    event_id,
    &event_organizer,
    0,
  ).await;

  // Create a new ticket sale for the first ticket type
  
}
