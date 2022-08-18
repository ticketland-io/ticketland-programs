use std::sync::{Arc};
use anchor_lang::{
  prelude::Result as AnchorResult,
  InstructionData,
  ToAccountMetas,
};
use solana_test_utils::{
  program_test::ProgramTest,
};
use solana_program_test::{tokio::sync::{Mutex}};
use solana_sdk::{
  system_program,
  sysvar::SysvarId,
  rent::{Rent},
  pubkey::Pubkey,
  signature::{Keypair, Signer},
  instruction::Instruction,
  native_token::sol_to_lamports,
};
use crate::{
  program_id::{
    secondary_market_program_id,
    event_registry_program_id,
  },
};

pub struct Runner {
  pub pt: Arc<Mutex<ProgramTest>>,
  pub deployer: Keypair,
}

impl Runner {
  pub async fn new(pt: Arc<Mutex<ProgramTest>>) -> Self {
    let mut pt_lock = pt.lock().await;
    let deployer = pt_lock.create_account(sol_to_lamports(1000_f64), 0, &system_program::ID).await;

    Self {
      pt: Arc::clone(&pt),
      deployer,
    }
  }

  pub async fn process_transaction(
    &self,
    instructions: &[Instruction],
    signers: Option<&[&Keypair]>,
  ) ->  AnchorResult<()> {
    let mut pt = self.pt.lock().await;
    pt.process_transaction(instructions, signers).await.map_err(Into::into)
  }

  pub async fn initialize(
    &mut self,
    state: &Keypair,
    event_registry_state: Pubkey,
    ticket_sale_state: Pubkey,
		protocol_fee: u16,
  ) {
    let accounts = secondary_market::accounts::Initialize {
      state: state.pubkey(),
      deployer: self.deployer.pubkey(),
      system_program: system_program::ID,
      rent: Rent::id(),
    }.to_account_metas(None);

    let data = secondary_market::instruction::Initialize {
      event_registry_state,
      event_registry_program: event_registry_program_id(),
      ticket_sale_state,
      ticket_sale_program: secondary_market_program_id(),
      protocol_fee
    }.data();

    let ix = Instruction {
      program_id: secondary_market_program_id(),
      accounts,
      data,
    };

    let mut lock_pt = self.pt.lock().await;
    assert!(lock_pt.process_transaction(&[ix], Some(&[&self.deployer, &state])).await.is_ok());
  }
}
