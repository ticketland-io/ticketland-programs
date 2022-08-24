use std::sync::{Arc};
use anchor_lang::{
  prelude::Result as AnchorResult,
  InstructionData,
  ToAccountMetas,
  Id,
};
use anchor_spl::token::{Token};
use solana_test_utils::{
  program_test::ProgramTest,
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
  native_token::sol_to_lamports,
};
use crate::{
  program_id::{
    secondary_market_program_id,
    event_registry_program_id,
    ticket_nft_program_id,
  },
  ticket_nft::{
    pda as TicketNftPda,
  },
  event_registry::{
    pda as EventRegistryPda,
  },
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
    let spl = Spl::new(Arc::clone(&pt));
    let deployer = pt_lock.create_account(sol_to_lamports(1000_f64), 0, &system_program::ID).await;

    Self {
      pt: Arc::clone(&pt),
      spl,
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
    ticket_nft_state: Pubkey,
    treasury: Pubkey,
		protocol_fee: u16,
  ) {
    let accounts = secondary_market::accounts::Initialize {
      state: state.pubkey(),
      deployer: self.deployer.pubkey(),
      cpi_authority: pda::cpi_authority(&state.pubkey()).0,
      system_program: system_program::ID,
      rent: Rent::id(),
    }.to_account_metas(None);

    let data = secondary_market::instruction::Initialize {
      event_registry_state,
      event_registry_program: event_registry_program_id(),
      ticket_sale_state,
      ticket_sale_program: secondary_market_program_id(),
      ticket_nft_state,
      ticket_nft_program: ticket_nft_program_id(),
      treasury,
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

  pub async fn create_market(
    &self,
    state: Pubkey,
    event_registry_state: Pubkey,
    event_id: [u8; 32],
    event_organizer: &Keypair,
    organizer_resale_fee: u16,
    resale_cap: u16,
  ) -> AnchorResult<()> {
    let event = EventRegistryPda::event(&event_registry_state, event_id).0;
    let market = pda::market(&state, event_id).0;
    
    let accounts = secondary_market::accounts::CreateMarket {
      state,
      event,
      market,
      event_organizer: event_organizer.pubkey(),
      system_program: system_program::ID,
      rent: Rent::id(),
    }.to_account_metas(None);

    let data = secondary_market::instruction::CreateMarket {
      event_id,
      organizer_resale_fee,
      resale_cap,
    }.data();

    let ix = Instruction {
      program_id: secondary_market_program_id(),
      accounts,
      data,
    };

    self.process_transaction(&[ix], Some(&[&event_organizer])).await
  }

  pub async fn create_sell_listing(
    &self,
    event_id: [u8; 32],
    ask_price: u64,
    state: Pubkey,
    event_registry_state: Pubkey,
    sale: Pubkey,
    seat_index: u32,
    ticket_nft_program_state: Pubkey,
    purchase_token: Pubkey,
    ticket_owner: &Keypair,
  ) -> AnchorResult<()> {
    let market = pda::market(&state, event_id).0;
    let event = EventRegistryPda::event(&event_registry_state, event_id).0;
    let ticket_nft = TicketNftPda::ticket_nft(&ticket_nft_program_state, seat_index, event_id).0;
    let ticket_metadata = TicketNftPda::ticket_metadata(&ticket_nft_program_state, &ticket_nft).0;
    let sell_listing = pda::sell_listing(&state, event_id, &ticket_metadata).0;

    let accounts = secondary_market::accounts::CreateSellListing {
      state,
      sale,
      event,
      market,
      sell_listing,
      ticket_metadata,
      purchase_token,
      ticket_owner_purchase_token_ata: Spl::get_associated_token_address(&ticket_owner.pubkey(), &purchase_token),
      ticket_owner: ticket_owner.pubkey(),

      token_program: Token::id(),
      associated_token_program: spl_associated_token_account::ID,
      system_program: system_program::ID,
      rent: Rent::id(),
    }.to_account_metas(None);

    let data = secondary_market::instruction::CreateSellListing {
      _ticket_nft: ticket_nft,
      event_id,
      ask_price,
    }.data();

    let ix = Instruction {
      program_id: secondary_market_program_id(),
      accounts,
      data,
    };

    self.process_transaction(&[ix], Some(&[&ticket_owner])).await
  }

  pub async fn fill_sell_listing(
    &self,
    event_id: [u8; 32],
    state: Pubkey,
    event_registry_state: Pubkey,
    sale: Pubkey,
    seat_index: u32,
    ticket_nft_program_state: Pubkey,
    purchase_token: Pubkey,
    treasury: Pubkey,
    ticket_owner: Pubkey,
    ticket_buyer: &Keypair,
    event_organizer: Pubkey,
  ) -> AnchorResult<()> {
    let market = pda::market(&state, event_id).0;
    let event = EventRegistryPda::event(&event_registry_state, event_id).0;
    let ticket_nft = TicketNftPda::ticket_nft(&ticket_nft_program_state, seat_index, event_id).0;
    let ticket_metadata = TicketNftPda::ticket_metadata(&ticket_nft_program_state, &ticket_nft).0;
    let sell_listing = pda::sell_listing(&state, event_id, &ticket_metadata).0;

    let accounts = secondary_market::accounts::FillSellListing {
      state,
      ticket_nft_program_state,
      sell_listing,
      event,
      market,
      sale,
      cpi_authority: pda::cpi_authority(&state).0,
      ticket_metadata,
      purchase_token,
      ticket_owner,
      ticket_owner_purchase_token_ata: Spl::get_associated_token_address(&ticket_owner, &purchase_token),
      event_organizer,
      event_organizer_purchase_token_ata: Spl::get_associated_token_address(&event_organizer, &purchase_token),
      treasury,
      service_fee_ata: Spl::get_associated_token_address(&treasury, &purchase_token),
      ticket_buyer: ticket_buyer.pubkey(),
      ticket_buyer_ata: Spl::get_associated_token_address(&ticket_buyer.pubkey(), &purchase_token),
      ticket_nft_program: ticket_nft_program_id(),
      token_program: Token::id(),
      associated_token_program: spl_associated_token_account::ID,
    }.to_account_metas(None);

    let data = secondary_market::instruction::FillSellListing {
      _event_id: event_id,
    }.data();

    let ix = Instruction {
      program_id: secondary_market_program_id(),
      accounts,
      data,
    };

    self.process_transaction(&[ix], Some(&[&ticket_buyer])).await
  }

  pub async fn create_buy_listing(
    &self,
    event_id: [u8; 32],
    bid_price: u64,
    state: Pubkey,
    event_registry_state: Pubkey,
    purchase_token: Pubkey,
    ticket_buyer: &Keypair,
    n_listing: u16,
  ) -> AnchorResult<()> {
    let event = EventRegistryPda::event(&event_registry_state, event_id).0;
    let buy_listing = pda::buy_listing(&state, event_id, &ticket_buyer.pubkey(), n_listing).0;
    let listing_escrow = pda::listing_escrow(&state, event_id, &buy_listing).0;

    let accounts = secondary_market::accounts::CreateBuyListing {
      state,
      event,
      buyer_data: pda::buyer_data(&state, event_id, &ticket_buyer.pubkey()).0,
      buy_listing,
      listing_escrow,
      listing_escrow_ata: Spl::get_associated_token_address(&listing_escrow, &purchase_token),
      purchase_token,
      ticket_buyer_ata: Spl::get_associated_token_address(&ticket_buyer.pubkey(), &purchase_token),
      ticket_buyer: ticket_buyer.pubkey(),
      token_program: Token::id(),
      associated_token_program: spl_associated_token_account::ID,
      system_program: system_program::ID,
      rent: Rent::id(),
    }.to_account_metas(None);

    let data = secondary_market::instruction::CreateBuyListing {
      _event_id: event_id,
      bid_price,
    }.data();

    let ix = Instruction {
      program_id: secondary_market_program_id(),
      accounts,
      data,
    };

    self.process_transaction(&[ix], Some(&[&ticket_buyer])).await
  }

  pub async fn fill_buy_listing(
    &self,
    event_id: [u8; 32],
    state: Pubkey,
    event_registry_state: Pubkey,
    ticket_nft_program_state: Pubkey,
    sale: Pubkey,
    purchase_token: Pubkey,
    treasury: Pubkey,
    ticket_buyer: Pubkey,
    event_organizer: Pubkey,
    ticket_owner: &Keypair,
    n_listing: u16,
    seat_index: u32,
  ) -> AnchorResult<()> {
    let event = EventRegistryPda::event(&event_registry_state, event_id).0;
    let market = pda::market(&state, event_id).0;
    let buy_listing = pda::buy_listing(&state, event_id, &ticket_buyer, n_listing).0;
    let listing_escrow = pda::listing_escrow(&state, event_id, &buy_listing).0;
    let ticket_nft = TicketNftPda::ticket_nft(&ticket_nft_program_state, seat_index, event_id).0;
    let ticket_metadata = TicketNftPda::ticket_metadata(&ticket_nft_program_state, &ticket_nft).0;

    let accounts = secondary_market::accounts::FillBuyListing {
      state,
      ticket_nft_program_state,
      buy_listing,
      event,
      market,
      sale,
      cpi_authority: pda::cpi_authority(&state).0,
      purchase_token,
      listing_escrow,
      listing_escrow_ata: Spl::get_associated_token_address(&listing_escrow, &purchase_token),
      ticket_buyer,
      ticket_metadata,
      event_organizer,
      event_organizer_purchase_token_ata: Spl::get_associated_token_address(&event_organizer, &purchase_token),
      service_fee_ata: Spl::get_associated_token_address(&treasury, &purchase_token),
      treasury,
      ticket_owner_purchase_token_ata: Spl::get_associated_token_address(&ticket_owner.pubkey(), &purchase_token),
      ticket_owner: ticket_owner.pubkey(),
      ticket_nft_program: ticket_nft_program_id(),
      token_program: Token::id(),
      associated_token_program: spl_associated_token_account::ID,
    }.to_account_metas(None);

    let data = secondary_market::instruction::FillBuyListing {
      _n_listing: n_listing,
      _event_id: event_id,
    }.data();

    let ix = Instruction {
      program_id: secondary_market_program_id(),
      accounts,
      data,
    };

    self.process_transaction(&[ix], Some(&[&ticket_owner])).await
  }

  pub async fn get_ata_balances(
    &mut self,
    treasury: Pubkey,
    event_organizer: Pubkey,
    ticket_owner: Pubkey,
    mint_account: Pubkey,
  ) -> (u64, u64, u64) {
    let treasury_ata = Spl::get_associated_token_address(&treasury, &mint_account);
    let event_organizer_ata = Spl::get_associated_token_address(&event_organizer, &mint_account);
    let ticket_owner_ata = Spl::get_associated_token_address(&ticket_owner, &mint_account);
    
    (
      self.spl.get_token_account(treasury_ata).await.amount,
      self.spl.get_token_account(event_organizer_ata).await.amount,
      self.spl.get_token_account(ticket_owner_ata).await.amount,
    )
  }

  pub async fn get_listing_escrow_balance(
    &mut self,
    state: Pubkey,
    ticket_buyer: Pubkey,
    event_id: [u8; 32],
    n_listing: u16,
    mint_account: Pubkey,
  ) -> u64 {
    let buy_listing = pda::buy_listing(&state, event_id, &ticket_buyer, n_listing).0;
    let listing_escrow = pda::listing_escrow(&state, event_id, &buy_listing).0;
    let listing_escrow_ata = Spl::get_associated_token_address(&listing_escrow, &mint_account);

    self.spl.get_token_account(listing_escrow_ata).await.amount
  }
}
