use test_context::{AsyncTestContext};
use super::event_registry::runner::Runner as EventRegistryRunner;
use super::ticket_sale::runner::Runner as TicketSaleRunner;

pub struct TestContext {
  pub event_registry_runner: EventRegistryRunner,
  pub ticket_sale_runner: TicketSaleRunner,
}

#[async_trait::async_trait]
impl AsyncTestContext for TestContext {
  async fn setup() -> Self {
    TestContext {
      event_registry_runner: EventRegistryRunner::new().await,
      ticket_sale_runner: TicketSaleRunner::new().await
    }
  }

  async fn teardown(self) {
    // Perform any teardown you wish.
  }
}
