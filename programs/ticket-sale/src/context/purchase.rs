// use anchor_lang::prelude::*;
// use crate::{
//   ID,
//   account_data::{
//     state::*,
//     event_capacity::*,
//   },
//   utils::program_error::ErrorCode,
// };

// #[derive(Accounts)]
// pub struct Purchase<'info> {
//   #[account(mut)]
//   pub state: Account<'info, State>,

//   // The newly created event
//   #[account(
//     seeds = [
//       b"event",
//       state.event_registry_state.key().as_ref(),
//       &event_capacity.load()?.event_id.to_string().as_ref()
//     ],
//     bump
//   )]
//   pub event: Box<Account<'info, Event>>,

//   /// CHECK: The account that will hold the seats bitmap
//   #[account(
//     mut,
//     constraint = EventCapacity::owner() == ID @ ErrorCode::NotOwnedByThisProgram,
//   )]
//   pub event_capacity: AccountLoader<'info, EventCapacity>,
// }
