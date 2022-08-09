use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo};
use anchor_metaplex::{
  CreateMetadata,
  CreateMasterEdition,
  create_metadata,
  create_master_edition,
};
use crate::{
  context::create_event_nft::CreateEventNft,
};

fn mint_edition_token(ctx: &Context<CreateEventNft>, signer_seeds: &[&[&[u8]]]) -> Result<()> {
  let cpi_accounts = MintTo {
    mint: ctx.accounts.event_nft.to_account_info(),
    to: ctx.accounts.organizer_event_nft_ata.to_account_info(),
    authority: ctx.accounts.event_nft_authority.to_account_info(),
  };
  
  let cpi_program = ctx.accounts.token_program.to_account_info();
  let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
  
  token::mint_to(cpi_ctx, 1)
}

fn create_event_nft(
  ctx: &Context<CreateEventNft>,
  signer_seeds: &[&[&[u8]]],
  name: String,
  symbol: String,
  uri: String,
  seller_fee_basis_points: u16,
) -> Result<()> {
  let cpi_accounts = CreateMetadata {
    mint: *ctx.accounts.event_nft.clone(),
    mint_authority: ctx.accounts.event_nft_authority.clone(),
    metadata_account: ctx.accounts.metadata.clone(),
    payer: ctx.accounts.event_organizer.to_account_info(),
    update_authority: ctx.accounts.event_nft_authority.to_account_info(),
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
    mint: *ctx.accounts.event_nft.clone(),
    mint_authority: ctx.accounts.event_nft_authority.clone(),
    metadata_account: ctx.accounts.metadata.clone(),
    payer: ctx.accounts.event_organizer.to_account_info(),
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
  ctx: Context<CreateEventNft>,
  name: String,
  symbol: String,
  uri: String,
) -> Result<()> {
  let state = &ctx.accounts.state;
  let state_key = state.key();

  let seeds: &[&[u8]] = &[
    b"event_nft_authority", state_key.as_ref(),
    &[state.bumps.event_nft_authority]
  ];
  let signer_seeds:&[&[&[u8]]] = &[&seeds[..]];

  // We need this otherwise we get this error "Editions must have exactly one token"
  mint_edition_token(&ctx, signer_seeds)?;
  create_event_nft(&ctx, signer_seeds, name, symbol, uri, state.seller_fee_basis_points)?;

  Ok(())
}
