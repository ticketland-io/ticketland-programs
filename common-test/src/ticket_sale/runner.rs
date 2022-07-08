use std::{
  sync::{Arc},
};
use solana_test_utils::{
  program_test::ProgramTest,
  test_account::{TestAccount},
  spl_associated_token_account,
  spl::Spl,
  utils::{to_base},
};
use solana_program_test::{tokio::sync::{Mutex}};
use solana_sdk::{
  system_program,
  sysvar::SysvarId,
  rent::{Rent},
  pubkey::Pubkey,
  signature::{Keypair, Signer},
  instruction::Instruction,
};
use anchor_metaplex::{
  mpl_token_metadata::{
    pda::{
      find_metadata_account,
      find_master_edition_account,
    },
  },
};
use anchor_lang::{
  prelude::Result as AnchorResult,
  Id,
  InstructionData,
  ToAccountMetas
};
use ticket_sale::{
  account_data::state::*,
};
use super::pda;

pub struct Runner {
  pub pt: Arc<Mutex<ProgramTest>>,
  pub test_account: TestAccount,
  pub spl: Spl,
  pub deployer: Keypair,
}

impl Runner {
  pub async fn new() -> Self {
    let metadata_id = anchor_metaplex::mpl_token_metadata::ID;
    let mut program_test = solana_program_test::ProgramTest::new("ticket_sale", ticket_sale::id(), None);
    ProgramTest::add_program(&mut program_test, "mpl_token_metadata", metadata_id, None);
    program_test.set_compute_max_units(250_000);

    let mut pt = ProgramTest::start_new(program_test).await;
    let deployer = pt.create_account(0, &system_program::ID).await;
    let test_account = TestAccount::new(&mut pt, 10).await;
    let pt = Arc::new(Mutex::new(pt));
    let spl = Spl::new(Arc::clone(&pt));

    Self {
      pt,
      test_account,
      spl,
      deployer,
    }
  }
}
