use std::{
  sync::{Arc},
};
use solana_test_utils::{
  program_test::ProgramTest,
  spl::Spl,
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
use anchor_lang::{
  InstructionData,
  ToAccountMetas
};
use crate::program_id::{
  ticket_nft_program_id,
  ticket_sale_program_id,
  secondary_market_program_id,
};
use super::pda;

pub struct Runner {
  pub pt: Arc<Mutex<ProgramTest>>,
  pub spl: Spl,
  pub deployer: Keypair,
}

impl Runner {
  pub async fn new(pt: Arc<Mutex<ProgramTest>>) -> Self {
    let mut pt_lock = pt.lock().await;
    let deployer = pt_lock.create_account(sol_to_lamports(1000_f64), 0, &system_program::ID).await;
    let spl = Spl::new(Arc::clone(&pt));

    Self {
      pt: Arc::clone(&pt),
      spl,
      deployer,
    }
  }

  pub async fn initialize(
    &mut self,
    state: &Keypair,
    ticket_sale_state: Pubkey,
  ) {
    let accounts = ticket_nft::accounts::Initialize {
      state: state.pubkey(),
      nft_authority: pda::nft_authority(&state.pubkey()).0,
      deployer: self.deployer.pubkey(),
      system_program: system_program::ID,
      rent: Rent::id(),
    }.to_account_metas(None);

    let data = ticket_nft::instruction::Initialize {
      ticket_sale_state,
      ticket_sale_program: ticket_sale_program_id(),
    }.data();

    let ix = Instruction {
      program_id: ticket_nft_program_id(),
      accounts,
      data,
    };

    let mut lock_pt = self.pt.lock().await;
    assert!(lock_pt.process_transaction(&[ix], Some(&[&self.deployer, &state])).await.is_ok());
  }

  pub async fn set_secondary_market_state(
    &mut self,
    state: Pubkey,
    secondary_market_state: Pubkey,
  ) {
    let accounts = ticket_nft::accounts::SetSecondaryMarket {
      state,
      deployer: self.deployer.pubkey(),
    }.to_account_metas(None);

    let data = ticket_nft::instruction::SetSecondaryMarket {
      secondary_market_state,
      secondary_market_program: secondary_market_program_id(),
    }.data();

    let ix = Instruction {
      program_id: ticket_nft_program_id(),
      accounts,
      data,
    };

    let mut lock_pt = self.pt.lock().await;
    assert!(lock_pt.process_transaction(&[ix], Some(&[&self.deployer])).await.is_ok());
  }
}
