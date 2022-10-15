use std::{
  sync::{Arc},
};
use solana_test_utils::{
  program_test::ProgramTest,
  test_account::{TestAccount},
  spl_associated_token_account,
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
  Id,
  prelude::Result as AnchorResult,
  InstructionData,
  ToAccountMetas
};
use anchor_spl::token::{Token};
use anchor_metaplex::{
  mpl_token_metadata::{
    pda::{
      find_metadata_account,
    },
  },
};
use common::{
  crypto::mt::{
    create_seat_leaf,
    get_null_leaf,
  },
};
use crate::{
  event_registry::{
    pda as EventRegistryPda,
  },
  ticket_sale:: {
    pda as TickerSalePda,
  },
  ticket_nft::{
    pda as TicketNftPda,
  },
  program_id::{
    event_registry_program_id,
    ticket_sale_program_id,
    ticket_nft_program_id,
  },
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

  pub fn dummy_seat_name(seat_index: u32) -> String {
    format!("Seat-{}", seat_index)
  }

  pub fn create_ticket_type_mt(&self, seat_indexes: Vec<(u32, u32)>, n_tickets: u32) -> MerkleTree {
    let null_leaf = get_null_leaf();
    let mut seats = vec![null_leaf; n_tickets as usize];

    for seat_range in seat_indexes {
      for i in seat_range.0..seat_range.1 {
        seats[i as usize] = create_seat_leaf(i, &Self::dummy_seat_name(i));
      }
    }
    
    MerkleTree::new(seats.to_vec())
  }

  pub fn get_participant(&self, index: usize) -> Keypair {
    Keypair::from_bytes(self.test_account.participants[index].to_bytes().as_ref()).unwrap()
  }
  
  pub fn get_mint_operators(&self) -> Vec<Pubkey> {
    vec![self.get_participant(7).pubkey(), self.get_participant(8).pubkey()]
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
  ) {
    let cpi_authority = pda::cpi_authority(&state.pubkey()).0;

    let accounts = ticket_sale::accounts::Initialize {
      state: state.pubkey(),
      cpi_authority,
      deployer: self.deployer.pubkey(),
      system_program: system_program::ID,
      rent: Rent::id(),
    }.to_account_metas(None);

    let data = ticket_sale::instruction::Initialize {
      treasury: self.treasury.pubkey(),
      event_registry_state,
      event_registry_program: event_registry_program_id(),
      mint_operators: self.get_mint_operators(),
    }.data();

    let ix = Instruction {
      program_id: ticket_sale_program_id(),
      accounts,
      data,
    };

    let mut lock_pt = self.pt.lock().await;
    assert!(lock_pt.process_transaction(&[ix], Some(&[&self.deployer, &state])).await.is_ok());
  }

  pub async fn create_treasury_atas(&mut self, tokens: &Vec<Pubkey>,) {
    for token in tokens {
      let _ = self.spl.create_associated_account(
        &self.treasury.pubkey(), 
        &token
      ).await;
    }
  }

  pub async fn create_sale(
    &self,
    state: Pubkey,
    event_registry_state: Pubkey,
    event_id: [u8; 32],
    event_organizer: &Keypair,
    ticket_type_index: u8,
  ) -> AnchorResult<()> {
    let event = EventRegistryPda::event(&event_registry_state, event_id).0;
    let sale = TickerSalePda::ticket_sale_state(
      &state,
      ticket_type_index,
      event_id,
    ).0;

    let accounts = ticket_sale::accounts::CreateSale {
      state,
      event,
      sale,
      event_organizer: event_organizer.pubkey(),
      system_program: system_program::ID,
      rent: Rent::id(),
    }.to_account_metas(None);

    let data = ticket_sale::instruction::CreateSale {
      ticket_type_index,
      event_id,
    }.data();

    let ix = Instruction {
      program_id: ticket_sale::id(),
      accounts,
      data,
    };

    self.process_transaction(&[ix], Some(&[&event_organizer])).await
  }

  pub async fn verify_seat(
    &mut self,
    ticket_buyer: &Keypair,
    ticket_sale_state: Pubkey,
    event_id: [u8; 32],
    ticket_type_index: u8,
    seat_index: u32,
		seat_name: String,
    merkle_proof: Vec<[u8; 32]>,
  ) -> AnchorResult<()> {
    let sale = TickerSalePda::ticket_sale_state(&ticket_sale_state, ticket_type_index, event_id).0;
    let seat_verification = TickerSalePda::seat_verification(&sale, seat_index, &seat_name).0;

    let accounts = ticket_sale::accounts::VerifySeat {
      state: ticket_sale_state,
      sale,
      seat_verification,
      ticket_buyer: ticket_buyer.pubkey(),
      system_program: system_program::ID,
      rent: Rent::id(),
    }.to_account_metas(None);

    let data = ticket_sale::instruction::VerifySeat {seat_index, seat_name, merkle_proof}.data();

    let ix = Instruction {
      program_id: ticket_sale_program_id(),
      accounts,
      data,
    };

    self.process_transaction(&[ix], Some(&[&ticket_buyer])).await
  }

  pub async fn fixed_price_purchase(
    &mut self,
    ticket_buyer: &Keypair,
    event_registry_state: Pubkey,
    ticket_sale_state: Pubkey,
    event_capacity: Pubkey,
    purchase_token: Pubkey,
    event_organizer: Pubkey,
    ticket_nft_program_state: Pubkey,
    event_id: [u8; 32],
    ticket_type_index: u8,
    seat_index: u32,
		seat_name: String,
  ) -> AnchorResult<()> {
    let cpi_authority = TickerSalePda::cpi_authority(&ticket_sale_state).0;
    let ticket_nft = TicketNftPda::ticket_nft(&ticket_nft_program_state, seat_index, event_id, ticket_type_index).0;
    let event_nft = EventRegistryPda::event_nft(&event_registry_state, event_id).0;
    let sale = TickerSalePda::ticket_sale_state(&ticket_sale_state, ticket_type_index, event_id).0;
    let seat_verification = TickerSalePda::seat_verification(&sale, seat_index, &seat_name).0;

    let accounts = ticket_sale::accounts::FixedPricePurchase {
      state: ticket_sale_state,
      seat_verification,
      event: EventRegistryPda::event(&event_registry_state, event_id).0,
      sale: TickerSalePda::ticket_sale_state(&ticket_sale_state, ticket_type_index, event_id).0,
      cpi_authority,
      event_capacity,
      purchase_token,
      event_organizer_purchase_token_ata: Spl::get_associated_token_address(&event_organizer, &purchase_token),
      event_organizer,
      service_fee_ata: Spl::get_associated_token_address(&self.treasury.pubkey(), &purchase_token),
      treasury: self.treasury.pubkey(),
      ticket_buyer_ata: Spl::get_associated_token_address(&ticket_buyer.pubkey(), &purchase_token),
      ticket_buyer: ticket_buyer.pubkey(),
      ticket_nft_program_state,
      ticket_nft,
      nft_authority: TicketNftPda::nft_authority(&ticket_nft_program_state).0,
      ticket_metadata: TicketNftPda::ticket_metadata(&ticket_nft_program_state, &ticket_nft).0,
      ticket_nft_ata: Spl::get_associated_token_address(&cpi_authority, &ticket_nft),
      event_nft,
      event_nft_metadata: find_metadata_account(&event_nft).0,
      ticket_nft_program: ticket_nft_program_id(),
      token_program: Token::id(),
      associated_token_program: spl_associated_token_account::ID,
      system_program: system_program::ID,
      rent: Rent::id(),
    }.to_account_metas(None);

    let data = ticket_sale::instruction::FixedPricePurchase {seat_index, seat_name}.data();

    let ix = Instruction {
      program_id: ticket_sale_program_id(),
      accounts,
      data,
    };

    self.process_transaction(&[ix], Some(&[&ticket_buyer])).await
  }

  pub async fn free_purchase(
    &mut self,
    ticket_buyer: &Keypair,
    event_registry_state: Pubkey,
    ticket_sale_state: Pubkey,
    event_capacity: Pubkey,
    event_organizer: Pubkey,
    ticket_nft_program_state: Pubkey,
    event_id: [u8; 32],
    ticket_type_index: u8,
    seat_index: u32,
		seat_name: String,
  ) -> AnchorResult<()> {
    let cpi_authority = TickerSalePda::cpi_authority(&ticket_sale_state).0;
    let ticket_nft = TicketNftPda::ticket_nft(&ticket_nft_program_state, seat_index, event_id, ticket_type_index).0;
    let event_nft = EventRegistryPda::event_nft(&event_registry_state, event_id).0;
    let sale = TickerSalePda::ticket_sale_state(&ticket_sale_state, ticket_type_index, event_id).0;
    let seat_verification = TickerSalePda::seat_verification(&sale, seat_index, &seat_name).0;

    let accounts = ticket_sale::accounts::FreePurchase {
      state: ticket_sale_state,
      seat_verification,
      event: EventRegistryPda::event(&event_registry_state, event_id).0,
      sale: TickerSalePda::ticket_sale_state(&ticket_sale_state, ticket_type_index, event_id).0,
      cpi_authority,
      event_capacity,
      event_organizer,
      ticket_buyer: ticket_buyer.pubkey(),
      ticket_nft_program_state,
      ticket_nft,
      nft_authority: TicketNftPda::nft_authority(&ticket_nft_program_state).0,
      ticket_metadata: TicketNftPda::ticket_metadata(&ticket_nft_program_state, &ticket_nft).0,
      ticket_nft_ata: Spl::get_associated_token_address(&cpi_authority, &ticket_nft),
      event_nft,
      event_nft_metadata: find_metadata_account(&event_nft).0,
      ticket_nft_program: ticket_nft_program_id(),
      token_program: Token::id(),
      associated_token_program: spl_associated_token_account::ID,
      system_program: system_program::ID,
      rent: Rent::id(),
    }.to_account_metas(None);

    let data = ticket_sale::instruction::FreePurchase {seat_index, seat_name}.data();

    let ix = Instruction {
      program_id: ticket_sale_program_id(),
      accounts,
      data,
    };

    self.process_transaction(&[ix], Some(&[&ticket_buyer])).await
  }

  pub async fn operator_purchase(
    &mut self,
    ticket_buyer: &Keypair,
    recipient: Pubkey,
    event_registry_state: Pubkey,
    ticket_sale_state: Pubkey,
    event_capacity: Pubkey,
    event_organizer: Pubkey,
    ticket_nft_program_state: Pubkey,
    event_id: [u8; 32],
    ticket_type_index: u8,
    seat_index: u32,
		seat_name: String,
  ) -> AnchorResult<()> {
    let cpi_authority = TickerSalePda::cpi_authority(&ticket_sale_state).0;
    let ticket_nft = TicketNftPda::ticket_nft(&ticket_nft_program_state, seat_index, event_id, ticket_type_index).0;
    let event_nft = EventRegistryPda::event_nft(&event_registry_state, event_id).0;
    let sale = TickerSalePda::ticket_sale_state(&ticket_sale_state, ticket_type_index, event_id).0;
    let seat_verification = TickerSalePda::seat_verification(&sale, seat_index, &seat_name).0;

    let accounts = ticket_sale::accounts::OperatorPurchase {
      state: ticket_sale_state,
      seat_verification,
      event: EventRegistryPda::event(&event_registry_state, event_id).0,
      sale: TickerSalePda::ticket_sale_state(&ticket_sale_state, ticket_type_index, event_id).0,
      cpi_authority,
      event_capacity,
      event_organizer,
      ticket_buyer: ticket_buyer.pubkey(),
      ticket_nft_program_state,
      ticket_nft,
      nft_authority: TicketNftPda::nft_authority(&ticket_nft_program_state).0,
      ticket_metadata: TicketNftPda::ticket_metadata(&ticket_nft_program_state, &ticket_nft).0,
      ticket_nft_ata: Spl::get_associated_token_address(&cpi_authority, &ticket_nft),
      event_nft,
      event_nft_metadata: find_metadata_account(&event_nft).0,
      ticket_nft_program: ticket_nft_program_id(),
      token_program: Token::id(),
      system_program: system_program::ID,
      rent: Rent::id(),
    }.to_account_metas(None);

    let data = ticket_sale::instruction::OperatorPurchase {seat_index, seat_name, recipient}.data();

    let ix = Instruction {
      program_id: ticket_sale_program_id(),
      accounts,
      data,
    };

    self.process_transaction(&[ix], Some(&[&ticket_buyer])).await
  }
}
