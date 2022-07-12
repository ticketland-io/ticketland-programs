use std::{
  sync::{Arc},
};
use solana_test_utils::{
  program_test::ProgramTest,
  test_account::{TestAccount},
  spl::Spl,
  merkle_tree::MerkleTree,
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
use common::{
  crypto::mt::{create_seat_leaf, get_null_leaf}
};
use ticket_sale::{
  account_data::event_capacity::MAX_VENUE_CAPACITY,
};
use crate::program_id::{
  event_registry_program_id,
  ticket_sale_program_id
};
use super::pda;

pub struct Runner {
  pub pt: Arc<Mutex<ProgramTest>>,
  pub test_account: TestAccount,
  pub spl: Spl,
  pub deployer: Keypair,
  pub treasury: Keypair,
}

impl Runner {
  pub async fn new(pt: Arc<Mutex<ProgramTest>>) -> Self {
    let mut pt_lock = pt.lock().await;
    let deployer = pt_lock.create_account(sol_to_lamports(1000_f64), 0, &system_program::ID).await;
    let treasury = pt_lock.create_account(sol_to_lamports(1000_f64), 0, &system_program::ID).await;
    let test_account = TestAccount::new(&mut pt_lock, 10).await;
    let spl = Spl::new(Arc::clone(&pt));

    Self {
      pt: Arc::clone(&pt),
      test_account,
      spl,
      deployer,
      treasury,
    }
  }

  pub fn create_ticket_type_mt(&self, seat_indexes: Vec<(u32, u32)>,) -> MerkleTree {
    let null_leaf = get_null_leaf();
    let mut seats = [null_leaf; MAX_VENUE_CAPACITY];

    for seat_range in seat_indexes {
      for i in seat_range.0..seat_range.1 {
        seats[i as usize] = create_seat_leaf(i, &format!("Seat-{}", i));
      }
    }
    
    MerkleTree::new(seats.to_vec())
  }

  pub async fn initialize(
    &mut self,
    state: &Keypair,
    event_registry_state: Pubkey,
  ) {
    let cpi_authority = pda::cpi_authority(&state.pubkey()).0;

    let accounts = ticket_sale::accounts::Initialize {
      state: state.pubkey(),
      event_registry_state,
      event_registry_program: event_registry_program_id(),
      cpi_authority,
      deployer: self.deployer.pubkey(),
      system_program: system_program::ID,
      rent: Rent::id(),
    }.to_account_metas(None);

    let data = ticket_sale::instruction::Initialize {
      treasury: self.treasury.pubkey(),
    }.data();

    let ix = Instruction {
      program_id: ticket_sale_program_id(),
      accounts,
      data,
    };

    let mut lock_pt = self.pt.lock().await;
    assert!(lock_pt.process_transaction(&[ix], Some(&[&self.deployer, &state])).await.is_ok());
  }
}
