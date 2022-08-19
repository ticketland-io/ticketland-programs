use anchor_lang::prelude::*;
use anchor_safe_math::SafeMath;
use anchor_spl::token::{self, Token, Transfer};
use ticket_nft::{
  program::TicketNft,
};
use crate::{
  account_data::{
    state::*,
    market::*,
  },
};

fn transfer_token<'info>(
  token_program: Program<'info, Token>,
  from: AccountInfo<'info>,
  to: AccountInfo<'info>,
  authority: AccountInfo<'info>,
  amount: u64,
) -> Result<()> {
  let cpi_accounts = Transfer::<'info> {
    from,
    to,
    authority,
  };
  let cpi_program = token_program.to_account_info();
  let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

  token::transfer(cpi_ctx, amount)
}

pub fn transfer_funds<'info>(
  price: u64,
  state: &Box<Account<State>>,
  market: &Box<Account<Market>>,
  token_program: &Program<'info, Token>,
  from: AccountInfo<'info>,
  service_fee_ata: AccountInfo<'info>,
  event_organizer_purchase_token_ata: AccountInfo<'info>,
  ticket_owner_purchase_token_ata: AccountInfo<'info>,
  authority: AccountInfo<'info>,
) -> Result<()> {
  let service_fee_amount = price
  .safe_mul(10_000_u16.safe_sub(state.protocol_fee)? as u64)?
  .safe_div(10_000)?;

  let event_organizer_amount = price
  .safe_mul(10_000_u16.safe_sub(market.organizer_resale_fee)? as u64)?
  .safe_div(10_000)?;

  let seller_amount = price
  .safe_sub(event_organizer_amount)?
  .safe_sub(service_fee_amount)?;

  // transfer to treasury
  if service_fee_amount > 0 {
    transfer_token(
      token_program.clone(),
      from.clone(),
      service_fee_ata.clone(),
      authority.clone(),
      service_fee_amount,
    )?;
  }

  // transfer to event organizer
  if event_organizer_amount > 0 {
    transfer_token(
      token_program.clone(),
      from.clone(),
      event_organizer_purchase_token_ata.clone(),
      authority.clone(),
      event_organizer_amount,
    )?;
  }

  // transfer to ticket seller
  if seller_amount > 0 {
    transfer_token(
      token_program.clone(),
      from.clone(),
      ticket_owner_purchase_token_ata.clone(),
      authority.clone(),
      seller_amount,
    )?;
  }

  Ok(())
}

pub fn change_ticket_ownership<'info>(
  state: &Box<Account<State>>,
  ticket_nft_program: &Program<'info, TicketNft>,
  ticket_nft_program_state: AccountInfo<'info>,
  ticket_metadata: AccountInfo<'info>,
  cpi_authority: AccountInfo<'info>,
  new_owner: Pubkey,
) -> Result<()> {
  let cpi_program = ticket_nft_program.to_account_info();
  let cpi_accounts = ticket_nft::cpi::accounts::Transfer {
    state: ticket_nft_program_state.to_account_info(),
    ticket_metadata: ticket_metadata.to_account_info(),
    secondary_market_cpi_authority:  cpi_authority.to_account_info(),
  };

  let state_key = state.key();
  let seeds: &[&[u8]] = &[
    b"market:cpi_authority", state_key.as_ref(),
    &[state.bumps.cpi_authority]
  ];
  let signer_seeds:&[&[&[u8]]] = &[&seeds[..]];
  let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

  ticket_nft::cpi::transfer(
    cpi_ctx,
    new_owner,
  )?;

  Ok(())
}
