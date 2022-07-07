use test_context::{AsyncTestContext};
use super::runner::Runner;

pub struct TestContext {
  pub runner: Runner,
}

#[async_trait::async_trait]
impl AsyncTestContext for TestContext {
  async fn setup() -> Self {
    TestContext {
      runner: Runner::new().await
    }
  }

  async fn teardown(self) {
    // Perform any teardown you wish.
  }
}
