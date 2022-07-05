use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo};
use anchor_metaplex::{
  CreateMetadata,
  CreateMasterEdition,
  create_metadata,
  create_master_edition,
  UpdatePrimarySaleHappenedViaToken,
  update_primary_sale_happened_via_token,
};
use crate::{
  context::create_event::CreateEvent, 
  account_data::event::*,
};

fn mint_edition_token(ctx: &Context<CreateEvent>, signer_seeds: &[&[&[u8]]]) -> Result<()> {
  let cpi_accounts = MintTo {
    mint: ctx.accounts.event_nft.to_account_info(),
    to: ctx.accounts.event_nft_authority_ata.to_account_info(),
    authority: ctx.accounts.event_nft_authority.to_account_info(),
  };
  
  let cpi_program = ctx.accounts.token_program.to_account_info();
  let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
  
  token::mint_to(cpi_ctx, 1)
}

fn create_event_nft(
  ctx: &Context<CreateEvent>,
  signer_seeds: &[&[&[u8]]],
  name: String,
  symbol: String,
  uri: String,
  seller_fee_basis_points: u16,
) -> Result<()> {
  let cpi_accounts = CreateMetadata {
    mint: ctx.accounts.event_nft.clone(),
    mint_authority: ctx.accounts.event_nft_authority.clone(),
    metadata_account: ctx.accounts.metadata.clone(),
    payer: ctx.accounts.creator.to_account_info(),
    update_authority: ctx.accounts.event_nft_authority_ata.to_account_info(),
    system_program: ctx.accounts.system_program.to_account_info(),
    rent: ctx.accounts.rent.to_account_info(),
  };

  create_metadata(
    cpi_accounts,
    signer_seeds,
    name.clone(),
    symbol.clone(),
    uri,
    None,
    seller_fee_basis_points,
    true,
    true,
    None,
    None
  )?;

  let cpi_accounts = CreateMasterEdition {
    edition: ctx.accounts.master_edition.clone(),
    mint: ctx.accounts.event_nft.clone(),
    mint_authority: ctx.accounts.event_nft_authority.clone(),
    metadata_account: ctx.accounts.metadata.clone(),
    payer: ctx.accounts.creator.to_account_info(),
    update_authority: ctx.accounts.event_nft_authority.to_account_info(),
    system_program: ctx.accounts.system_program.to_account_info(),
    rent: ctx.accounts.rent.to_account_info(),
  };

  create_master_edition(
    cpi_accounts,
    signer_seeds,
    Some(0),
  )
}

pub fn exec(
  ctx: Context<CreateEvent>,
  start_time: SLOT,
  end_time: SLOT,
  name: String,
  symbol: String,
  uri: String,
) -> Result<()> {
  
  todo!()
}
