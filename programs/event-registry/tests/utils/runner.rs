use std::{
  sync::{Arc},
};
use anchor_lang::{
  prelude::Result as AnchorResult,
  Id,
  InstructionData,
  ToAccountMetas
};
use solana_test_utils::{
  program_test::ProgramTest,
  test_account::{TestAccount},
  spl_associated_token_account,
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
};
use anchor_metaplex::{
  mpl_token_metadata::{
    pda::{
      find_metadata_account,
      find_master_edition_account,
    },
  },
};
use event_registry::{
  account_data::state::*,
};
use super::pda;

pub struct Runner {
  pub pt: Arc<Mutex<ProgramTest>>,
  pub test_account: TestAccount,
  pub spl: Spl,
  pub deployer: Keypair,
  pub supported_currencies: Vec<Currency>,
  pub deposit_tokens: Vec<Pubkey>,
}

impl Runner {
  pub async fn new() -> Self {
    let metadata_id = anchor_metaplex::mpl_token_metadata::ID;
    let mut program_test = solana_program_test::ProgramTest::new("event_registry", event_registry::id(), None);
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
      supported_currencies: vec![],
      deposit_tokens: vec![],
    }
  }

  async fn create_deposit_tokens(&mut self) { 
    let mut deposit_tokens = vec![];

    for i in 0..2 {
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
        1_000_000 * (1e6 as u64)
      ).await;

      deposit_tokens.push(mint_token.pubkey());
    }

    let mut supported_currencies = vec![];

    for mint_account in &deposit_tokens {
      supported_currencies.push(Currency {
        mint_account: *mint_account,
        deposit_amount: 1_000 * (1e6 as u64), // 1000 USDC for example
      })
    }

    self.deposit_tokens = deposit_tokens;
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
		service_fee: u16,
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
      service_fee,
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

  pub async fn create_event(
    &self,
    state: Pubkey,
    event_id: u64,
    deposit_token: Pubkey,
    event_organizer: &Keypair,
    start_time: Slot,
		end_time: Slot,
		ticket_types: Vec<TicketType>,
		name: String,
		symbol: String,
		uri: String,
  ) -> AnchorResult<()> {
    let event_nft = pda::event_nft(&state, event_id).0;
    let event_nft_authority = pda::event_nft_authority(&state).0;
    let fund_manager = pda::fund_manager(&sate, &event_organizer.pubkey()).0;
    
    let accounts = event_registry::accounts::CreateEvent {
      state,
      event: pda::event(&state, event_id).0,
      event_nft,
      event_nft_authority,
      event_nft_authority_ata: pda::event_nft_authority_ata(&event_nft_authority, &event_nft),
      metadata: find_metadata_account(&event_nft).0,
      master_edition: find_master_edition_account(&event_nft).0,
      deposit_token,
      fund_manager,
      fund_manager_ata: pda::fund_manager_ata(&fund_manager, &deposit_token),
      event_organizer: event_organizer.pubkey(),

      // metadata_program: anchor_metaplex::mpl_token_metadata::ID,
      token_program: Token::id(),
      associated_token_program: spl_associated_token_account::ID,
      system_program: system_program::ID,
      rent: Rent::id(),
    }.to_account_metas(None);

    let data = event_registry::instruction::CreateEvent {
      start_time,
      end_time,
      ticket_types,
      name,
      symbol,
      uri,
    }.data();
    
    let ix = Instruction {
      program_id: event_registry::id(),
      accounts,
      data,
    };

    Self::process_transaction(&[ix], Some(&[&event_organizer])).await
  }
}
