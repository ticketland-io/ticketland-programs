// use anchor_lang::prelude::*;
// use std::mem::size_of;
// use anchor_spl::{
//   token::{Mint, Token, TokenAccount},
//   associated_token::AssociatedToken,
// };
// use crate::{
//   account_data::{
//     state::*,
//     sell_listing::*,
//     market::Market,
//   },
// };

// #[derive(Accounts)]
// #[instruction(ticket_nft: Pubkey, market_id: [u8; 32], event_id: [u8; 32])]
// pub struct CreateSellListing<'info> {
//   #[account(mut)]
//   pub state: Account<'info, State>,

//   // The sell listing account
//   #[account(
//     init,
//     space = 8 + size_of::<SellListing>(),
//     payer = ticket_owner,
//     seeds = [
//       b"sell_listing",
//       state.key().as_ref(),
//       &market_id,
//       &event_id,
//       ticket_nft.as_ref(),
//     ],
//     bump,
//   )]
//   pub sell_listing: Account<'info, SellListing>,


//   // The market account
//   #[account(
//     init,
//     space = 8 + size_of::<Market>() + market::SPACE_MARGIN,
//     payer = event_organizer,
//     seeds = [
//       b"market",
//       state.key().as_ref(),
//       &event_id,
//     ],
//     bump,
//   )]
//   pub market: Account<'info, Market>,
// }
