use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo};
use common::{
  utils::string::puffed_out_string,
};
use crate::{
  account_data::ticket_metadata::{MAX_NAME_LENGTH},
  context::create_ticket::CreateTicket,
};

/// Will mint a new NFT token and transfer it to the ticket_nft_ata controlled by the PDA ticket_sale_cpi_authority
/// owned by the Ticket sale program
fn mint_ticket_nft(ctx: &Context<CreateTicket>, signer_seeds: &[&[&[u8]]]) -> Result<()> {
  let cpi_accounts = MintTo {
    mint: ctx.accounts.nft.to_account_info(),
    to: ctx.accounts.ticket_nft_ata.to_account_info(),
    authority: ctx.accounts.nft_authority.to_account_info(),
  };
  
  let cpi_program = ctx.accounts.token_program.to_account_info();
  let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
  
  token::mint_to(cpi_ctx, 1)
}

pub fn exec(
  ctx: Context<CreateTicket>,
  event_id: [u8; 32],
  sale: Pubkey,
  price_sold: u64,
  seat_index: u32,
  recipient: Pubkey,
  name: String,
) -> Result<()> {  
  let ticket_metadata = &mut ctx.accounts.ticket_metadata;

  ticket_metadata.mint = ctx.accounts.nft.key();
  ticket_metadata.collection = ctx.accounts.event_nft_metadata.key();
  ticket_metadata.name = puffed_out_string(&name, MAX_NAME_LENGTH);
  ticket_metadata.event_id = event_id;
  ticket_metadata.seat_index = seat_index;
  ticket_metadata.sale = sale;
  ticket_metadata.price_sold = price_sold;
  ticket_metadata.owner = recipient;
  ticket_metadata.attended = false;

  let state = &mut ctx.accounts.state;
  let state_key = state.key();

  let seeds: &[&[u8]] = &[
    b"nft_authority", state_key.as_ref(),
    &[state.bumps.nft_authority]
  ];
  let signer_seeds:&[&[&[u8]]] = &[&seeds[..]];

  mint_ticket_nft(&ctx, signer_seeds)?;

  Ok(())
}
