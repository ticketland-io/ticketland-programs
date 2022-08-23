use anchor_lang::prelude::*;
use anchor_spl::{
  token::{Mint, Token, TokenAccount},
  associated_token::AssociatedToken,
};
use ticket_nft::{
  program::TicketNft,
};
use crate::{
  account_data::{
    state::*,
    sell_listing::*,
    market::Market,
  },
  utils::{
    program_error::ErrorCode,
  }
};

#[derive(Accounts)]
#[instruction(ticket_nft: Pubkey, event_id: [u8; 32])]
pub struct FillSellListing<'info> {
  #[account(mut)]
  pub state: Box<Account<'info, State>>,

  #[account(
    constraint = ticket_nft_program_state.key() == state.ticket_nft_state @ ErrorCode::WrongTicketNftState
  )]
  pub ticket_nft_program_state: Box<Account<'info, State>>,

  // The sell listing account
  // It will be closed right after the instruction is executed
  #[account(
    mut,
    close = ticket_owner,
    seeds = [
      b"sell_listing",
      state.key().as_ref(),
      &event_id,
      ticket_nft.as_ref(),
    ],
    bump,
  )]
  pub sell_listing: Box<Account<'info, SellListing>>,

  // The market account
  #[account(
    seeds = [
      b"market",
      state.key().as_ref(),
      &event_id,
    ],
    bump = market.bumps.market,
  )]
  pub market: Box<Account<'info, Market>>,

  /// CHECK: The Sale account stored in the ticket_metadata
  /// Processor will check that the ticket_metadata.sale does much this key
  #[account()]
  pub sale: AccountInfo<'info>,

  /// CHECK: THe PDA that will be sending CPI to other programs i.e. TicketSale Program
  #[account(
    seeds = [b"market:cpi_authority", state.key().as_ref()],
    bump = state.bumps.cpi_authority,
  )]
  pub cpi_authority: AccountInfo<'info>,

  /// CHECK: The ticket metadata account.
  #[account(
    constraint = ticket_metadata.key() == sell_listing.ticket_metadata @ ErrorCode::WrongTicketMetadata,
  )]
  pub ticket_metadata: AccountInfo<'info>,
  
  /// CHECK: The token that was used in the primary market of this event
  #[account()]
  pub purchase_token: Box<Account<'info, Mint>>,
    
  /// CHECK: The ticket seller
  #[account(
    mut,
    constraint = ticket_owner.key() == sell_listing.ticket_owner @ ErrorCode::WrongTicketSeller
  )]
  pub ticket_owner: AccountInfo<'info>,

  /// The ticket owner ata that will receive the funds from the ticket sell
  /// The last constraint will make sure that the purchase token account is correct. This is because
  /// When the sell listing is created the purchase token is verified that it is the same as the event currency; so
  /// if ATA constraints create the correct account then the purchase token is also correct
  #[account(
    mut,
    associated_token::mint = purchase_token,
    associated_token::authority = ticket_owner,
    constraint = sell_listing.ticket_owner_purchase_token_ata == ticket_owner_purchase_token_ata.key() @ ErrorCode::WrongTicketOwnerPurchaseTokenAta
  )]
  pub ticket_owner_purchase_token_ata: Box<Account<'info, TokenAccount>>,

  /// CHECK: This is the user that created the event earlier
  /// Check about the correctness of this account will take place in the Ticket sale before this CPI is called
  #[account(mut)]
  pub event_organizer: AccountInfo<'info>,

  /// The event organizer ATA that till be receiving the funds from the fees
  #[account(
    associated_token::mint = purchase_token,
    associated_token::authority = event_organizer,
  )]
  pub event_organizer_purchase_token_ata: Box<Account<'info, TokenAccount>>,

  /// The token token account that will be receiving the service fee from the resale
  #[account(
    associated_token::mint = purchase_token,
    associated_token::authority = treasury,
  )]
  pub service_fee_ata: Box<Account<'info, TokenAccount>>,

  /// CHECK: This is ticketland.io treasury address
  #[account()]
  pub treasury: AccountInfo<'info>,

  #[account(mut)]
  pub ticket_buyer: Signer<'info>,

  #[account(
    mut,
    associated_token::mint = purchase_token,
    associated_token::authority = ticket_buyer,
  )]
  pub ticket_buyer_ata: Box<Account<'info, TokenAccount>>,

  pub ticket_nft_program: Program<'info, TicketNft>,

  pub token_program: Program<'info, Token>,
  pub associated_token_program: Program<'info, AssociatedToken>,
}
