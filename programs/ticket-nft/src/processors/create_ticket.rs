use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo};
use anchor_metaplex::{
  CreateMetadata,
  CreateMasterEdition,
  create_metadata,
  create_master_edition,
  mpl_token_metadata::state::{Metadata},
};
use crate::{
  context::create_ticket::CreateTicket,
};

/// Will mint a new NFT token and transfer it to the ticket_nft_ata controlled by the PDA ticket_sale_cpi_authority
/// owned by the Ticket sale program
fn mint_edition_token(ctx: &Context<CreateTicket>, signer_seeds: &[&[&[u8]]]) -> Result<()> {
  let cpi_accounts = MintTo {
    mint: ctx.accounts.nft.to_account_info(),
    to: ctx.accounts.ticket_nft_ata.to_account_info(),
    authority: ctx.accounts.nft_authority.to_account_info(),
  };
  
  let cpi_program = ctx.accounts.token_program.to_account_info();
  let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
  
  token::mint_to(cpi_ctx, 1)
}

/// Create the Metaplex metadata
fn create_nft_metadata(
  ctx: &Context<CreateTicket>,
  signer_seeds: &[&[&[u8]]],
  name: String,
) -> Result<()> {
  let event_nft_metadata = Metadata::from_account_info(&ctx.accounts.event_nft_metadata)?;

  let cpi_accounts = CreateMetadata {
    mint: *ctx.accounts.nft.clone(),
    mint_authority: ctx.accounts.nft_authority.to_account_info(),
    metadata_account: ctx.accounts.metadata.clone(),
    payer: ctx.accounts.ticket_buyer.to_account_info(),
    update_authority: ctx.accounts.nft_authority.clone(),
    system_program: ctx.accounts.system_program.to_account_info(),
    rent: ctx.accounts.rent.to_account_info(),
  };

  create_metadata(
    cpi_accounts,
    signer_seeds,
    name,
    event_nft_metadata.data.symbol.trim_matches(char::from(0)).to_owned(),
    event_nft_metadata.data.uri.trim_matches(char::from(0)).to_owned(),
    None,
    0,
    true,
    false,
    None,
    None,
  )?;

  let cpi_accounts = CreateMasterEdition {
    edition: ctx.accounts.master_edition.clone(),
    mint: *ctx.accounts.nft.clone(),
    mint_authority: ctx.accounts.nft_authority.clone(),
    metadata_account: ctx.accounts.metadata.clone(),
    payer: ctx.accounts.ticket_buyer.to_account_info(),
    update_authority: ctx.accounts.nft_authority.to_account_info(),
    system_program: ctx.accounts.system_program.to_account_info(),
    rent: ctx.accounts.rent.to_account_info(),
  };

  create_master_edition(
    cpi_accounts,
    signer_seeds,
    Some(0),
  )?;

  Ok(())
}

pub fn exec(
  ctx: Context<CreateTicket>,
  event_id: [u8; 32],
  name: String,
) -> Result<()> {  
  let ticket_metadata = &mut ctx.accounts.ticket_metadata;

  ticket_metadata.event_id = event_id;
  ticket_metadata.metadata = ctx.accounts.metadata.key();
  ticket_metadata.owner = ctx.accounts.ticket_buyer.key();
  ticket_metadata.attended = false;

  let state = &mut ctx.accounts.state;
  let state_key = state.key();

  let seeds: &[&[u8]] = &[
    b"nft_authority", state_key.as_ref(),
    &[state.bumps.nft_authority]
  ];
  let signer_seeds:&[&[&[u8]]] = &[&seeds[..]];

  mint_edition_token(&ctx, signer_seeds)?;
  create_nft_metadata(&ctx, signer_seeds, name)?;

  Ok(())
}
