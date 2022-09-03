use std::{
  sync::{Arc},
};
use anchor_lang::{
  prelude::Result as AnchorResult,
  Id,
  InstructionData,
  ToAccountMetas,
  solana_program::program_option::COption,
};
use solana_test_utils::{
  program_test::ProgramTest,
  test_account::{TestAccount},
  spl_associated_token_account,
  spl::Spl,
  utils::{to_base},
  spl_token,
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
use anchor_spl::token::{Token};
use anchor_metaplex::{
  mpl_token_metadata::{
    pda::{
      find_metadata_account,
      find_master_edition_account,
    },
  },
};
use common::{
  state::{
    ticket_type::TicketType,
    currency::*,
  },
};
use ticket_sale::account_data::event_capacity::{
  EventCapacity,
  SPACE_MARGIN as event_capacity_space_margin,
};
use crate::{
  program_id::ticket_sale_program_id,
};
use super::pda;

pub struct Runner {
  pub pt: Arc<Mutex<ProgramTest>>,
  pub test_account: TestAccount,
  pub spl: Spl,
  pub deployer: Keypair,
  pub supported_currencies: Vec<Currency>,
  pub deposit_tokens: Vec<Pubkey>,
  pub deposit_token_authorities: Vec<Keypair>,
}

impl Runner {
  pub async fn new(pt: Arc<Mutex<ProgramTest>>) -> Self {
    let mut pt_lock = pt.lock().await;
    let deployer = pt_lock.create_account(sol_to_lamports(1000_f64), 0, &system_program::ID).await;
    let test_account = TestAccount::new(&mut pt_lock, 10).await;
    let spl = Spl::new(Arc::clone(&pt));

    Self {
      pt: Arc::clone(&pt),
      test_account,
      spl,
      deployer,
      supported_currencies: vec![],
      deposit_tokens: vec![],
      deposit_token_authorities: vec![],
    }
  }

  pub fn get_participant(&self, index: usize) -> Keypair {
    Keypair::from_bytes(self.test_account.participants[index].to_bytes().as_ref()).unwrap()
  }

  async fn create_deposit_tokens(&mut self) { 
    let mut deposit_tokens = vec![];
    let mut deposit_token_authorities = vec![];

    for _ in 0..2 {
      let mint_token = Keypair::new();
      let authority = Keypair::new();

      self.spl.create_mint(
        &mint_token,
        &authority.pubkey(),
        None,
        6
      ).await;

      self.spl.airdrop(
        &mint_token.pubkey(),
        &authority,
        &self.test_account.participants,
        to_base(1_000_000, 6),
      ).await;

      let _ = self.spl.create_associated_account(
        &self.deployer.pubkey(), 
        &mint_token.pubkey()
      ).await;

      deposit_tokens.push(mint_token.pubkey());
      deposit_token_authorities.push(authority);
    }

    let mut supported_currencies = vec![];

    for mint_account in &deposit_tokens {
      supported_currencies.push(Currency {
        mint_account: *mint_account,
        treasury_ata: Spl::get_associated_token_address(&self.deployer.pubkey(), &*mint_account),
        deposit_amount: to_base(1000, 6), // 1000 USDC for example
        service_fee: 500, // 5%
      })
    }

    // make sure the account is available in the test environment
    {
      let authority = Keypair::new();
      
      self.spl.set_mint_account(
        &spl_token::native_mint::id(),
        sol_to_lamports(1_f64),
        COption::None,
        to_base(1000000, 9),
        9,
      ).await;

      // Create an ATA for each participant
      for participant in &self.test_account.participants {
        self.spl.wrap_sol(&spl_token::native_mint::id(), &participant, to_base(100, 9)).await;
      }

      let _ = self.spl.create_associated_account(
        &self.deployer.pubkey(), 
        &spl_token::native_mint::id()
      ).await;

      deposit_token_authorities.push(authority);
    }

    supported_currencies.push(Currency {
      mint_account: spl_token::native_mint::id(),
      treasury_ata: Spl::get_associated_token_address(&self.deployer.pubkey(), &spl_token::native_mint::id()),
      deposit_amount: sol_to_lamports(10_f64),
      service_fee: 500, // 5%
    });

    deposit_tokens.push(spl_token::native_mint::id());
    self.deposit_tokens = deposit_tokens;
    self.deposit_token_authorities = deposit_token_authorities;
    self.supported_currencies = supported_currencies;
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
		seller_fee_basis_points: u16,
  ) {
    self.create_deposit_tokens().await;

    let accounts = event_registry::accounts::Initialize {
      state: state.pubkey(),
      event_nft_authority: pda::event_nft_authority(&state.pubkey()).0,
      cpi_authority: pda::cpi_authority(&state.pubkey()).0,
      deployer: self.deployer.pubkey(),
      system_program: system_program::ID,
      rent: Rent::id(),
    }.to_account_metas(None);

    let data = event_registry::instruction::Initialize {
      supported_currencies: self.supported_currencies.clone(),
      seller_fee_basis_points,
    }.data();

    let ix = Instruction {
      program_id: event_registry::id(),
      accounts,
      data,
    };

    let mut lock_pt = self.pt.lock().await;
    assert!(lock_pt.process_transaction(&[ix], Some(&[&self.deployer, &state])).await.is_ok());
  }

  pub async fn create_event_capacity_account(&mut self) -> Pubkey {
    let mut pt_lock = self.pt.lock().await;
    let space = 8 + std::mem::size_of::<EventCapacity>() + event_capacity_space_margin + 8;
    
    pt_lock.create_account(
      sol_to_lamports(1000_f64),
      space as u64, 
      &ticket_sale_program_id()
    ).await.pubkey()
  }

  pub async fn create_event(
    &self,
    state: Pubkey,
    event_capacity: Pubkey,
    ticket_sale_program_state: Pubkey,
    event_id: [u8; 32],
    deposit_token: Pubkey,
    purchase_token: Pubkey,
    event_organizer: &Keypair,
    n_tickets: u32,
    start_time: i64,
		end_time: i64,
		ticket_types: Vec<TicketType>,
  ) -> AnchorResult<()> {
    let event = pda::event(&state, event_id).0;
    let fund_manager = pda::fund_manager(&state, &event, &event_organizer.pubkey()).0;
    let cpi_authority = pda::cpi_authority(&state).0;

    let accounts = event_registry::accounts::CreateEvent {
      state,
      event,
      deposit_token,
      purchase_token,
      fund_manager,
      fund_manager_ata: pda::fund_manager_ata(&fund_manager, &deposit_token),
      event_organizer_ata: pda::event_organizer_ata(&event_organizer.pubkey(), &deposit_token),
      event_organizer_purchase_token_ata: Spl::get_associated_token_address(&event_organizer.pubkey(), &purchase_token),
      event_capacity,
      ticket_sale_program_state,
      cpi_authority,
      event_organizer: event_organizer.pubkey(),
      
      ticket_sale_program: ticket_sale_program_id(),
      token_program: Token::id(),
      associated_token_program: spl_associated_token_account::ID,
      system_program: system_program::ID,
      rent: Rent::id(),
    }.to_account_metas(None);

    let data = event_registry::instruction::CreateEvent {
      event_id,
      n_tickets,
      start_time,
      end_time,
      ticket_types,
    }.data();
    
    let ix = Instruction {
      program_id: event_registry::id(),
      accounts,
      data,
    };

    self.process_transaction(&[ix], Some(&[&event_organizer])).await
  }

  pub async fn create_event_nft(
    &self,
    state: Pubkey,
    event_id: [u8; 32],
    event_organizer: &Keypair,
		name: String,
		symbol: String,
		uri: String,
  ) -> AnchorResult<()> {
    let event = pda::event(&state, event_id).0;
    let event_nft = pda::event_nft(&state, event_id).0;
    let event_nft_authority = pda::event_nft_authority(&state).0;

    let accounts = event_registry::accounts::CreateEventNft {
      state,
      event,
      event_nft,
      event_nft_authority,
      organizer_event_nft_ata: pda::organizer_event_nft_ata(&event_organizer.pubkey(), &event_nft),
      metadata: find_metadata_account(&event_nft).0,
      master_edition: find_master_edition_account(&event_nft).0,
      event_organizer: event_organizer.pubkey(),
      metadata_program: anchor_metaplex::mpl_token_metadata::ID,
      token_program: Token::id(),
      associated_token_program: spl_associated_token_account::ID,
      system_program: system_program::ID,
      rent: Rent::id(),
    }.to_account_metas(None);

    let data = event_registry::instruction::CreateEventNft {
      _event_id: event_id,
      name,
      symbol,
      uri,
    }.data();
    
    let ix = Instruction {
      program_id: event_registry::id(),
      accounts,
      data,
    };

    self.process_transaction(&[ix], Some(&[&event_organizer])).await
  }

  pub async fn get_event_organizer_ata_balance(
    &mut self, 
    event_organizer: &Pubkey,
    mint_account: &Pubkey,
  ) -> u64 {
    let event_organizer_ata = pda::event_organizer_ata(&event_organizer, &mint_account);
    self.spl.get_token_account(event_organizer_ata).await.amount
  }

  pub async fn get_treasury_ata_balance(
    &mut self, 
    treasury: &Pubkey,
    mint_account: &Pubkey,
  ) -> u64 {
    let treasury_ata = Spl::get_associated_token_address(&treasury, &mint_account);
    self.spl.get_token_account(treasury_ata).await.amount
  }
}
