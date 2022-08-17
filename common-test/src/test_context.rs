use std::{
  sync::{Arc},
};
use test_context::{AsyncTestContext};
use solana_program_test::{tokio::sync::{Mutex}};
use solana_test_utils::{
  program_test::ProgramTest,
};
use super::{
  program_id::{
    event_registry_program_id,
    ticket_sale_program_id,
    ticket_nft_program_id,
    secondary_market_program_id,
  },
  event_registry::runner::Runner as EventRegistryRunner,
  ticket_sale::runner::Runner as TicketSaleRunner,
  ticket_nft::runner::Runner as TicketNftRunner,
  secondary_market::runner::Runner as SecondaryMarketRunner,
};

pub struct TestContext {
  pub event_registry_runner: EventRegistryRunner,
  pub ticket_sale_runner: TicketSaleRunner,
  pub ticket_nft_runner: TicketNftRunner,
  pub secondary_market_runner: SecondaryMarketRunner,
}

#[async_trait::async_trait]
impl AsyncTestContext for TestContext {
  async fn setup() -> Self {
    let metadata_id = anchor_metaplex::mpl_token_metadata::ID;
    let mut program_test = solana_program_test::ProgramTest::new("event_registry", event_registry_program_id(), None);
    
    ProgramTest::add_program(&mut program_test, "ticket_sale", ticket_sale_program_id(), None);
    ProgramTest::add_program(&mut program_test, "ticket_nft", ticket_nft_program_id(), None);
    ProgramTest::add_program(&mut program_test, "secondary_market", secondary_market_program_id(), None);
    ProgramTest::add_program(&mut program_test, "mpl_token_metadata", metadata_id, None);
    
    program_test.set_compute_max_units(500_000);

    let pt = ProgramTest::start_new(program_test).await;
    let pt = Arc::new(Mutex::new(pt));

    TestContext {
      event_registry_runner: EventRegistryRunner::new(Arc::clone(&pt)).await,
      ticket_sale_runner: TicketSaleRunner::new(Arc::clone(&pt)).await,
      ticket_nft_runner: TicketNftRunner::new(Arc::clone(&pt)).await,
      secondary_market_runner: SecondaryMarketRunner::new(Arc::clone(&pt)).await,
    }
  }

  async fn teardown(self) {
    // Perform any teardown you wish.
  }
}
