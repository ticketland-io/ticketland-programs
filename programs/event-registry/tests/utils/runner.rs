use solana_test_utils::{
  program_test::ProgramTest,
  test_account::{TestAccount},
  spl_associated_token_account,
  spl::Spl,
};
use solana_sdk::{
  system_program,
  sysvar::SysvarId,
  rent::{Rent},
  pubkey::Pubkey,
  signature::{Keypair, Signer},
  instruction::Instruction,
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
    let mut program_test = solana_program_test::ProgramTest::new("event_registry", jbas::id(), None);
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

  pub async fn initialize(
    &mut self,
    state: &Keypair,
		service_fee: u16,
		seller_fee_basis_points: u16,
  ) {
    self.create_deposit_tokens().await;

    let accounts = event_registry::accounts::Initialize {
      state: state.pubkey(),
      event_nft_authority: pda.event_nft_authority().0,
      cpi_authority: pda.cpi_authority().0,
      deployer: self.deployer.pubkey(),
      system_program: system_program::ID,
      rent: Rent::id(),
    }.to_account_metas(None);

    let data = event_registry::instruction::Initialize {
      supported_currencies: self.supported_currencies,
      service_fee,
      seller_fee_basis_points,
    }.data();

    let ix = Instruction {
      program_id: soliquid::id(),
      accounts,
      data,
    };

    assert!(lock_pt.process_transaction(&[ix], Some(&[&self.deployer, &state])).await.is_ok());
  }

  fn create_deposit_tokens(&mut self) { 
    let mut deposit_tokens = vec![];

    for i in 0..2 {
      let mint_token = Keypair::new();
      let authority = Keypair::new();

      self.spl.create_mint(
        &mint_token,
        &authority,
        None,
        6
      ).await;

      self.spl.airdrop(
        &mint_token.pubkey(),
        &authority,
        &self.test_accounts.participants,
        1_000_000 * 1e6
      ).await;

      deposit_tokens.push(mint_token.pubkey());
    }

    let mut supported_currencies = vec![];

    for mint_account in deposit_tokens {
      supported_currencies.push(Currency {
        mint_account,
        deposit_amount: 1_000 * 1e6, // 1000 USDC for example
      })
    }

    self.deposit_tokens = deposit_tokens;
    self.supported_currencies = supported_currencies;
  }
}
