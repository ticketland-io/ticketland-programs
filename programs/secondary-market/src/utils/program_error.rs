use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("Not event account")]
  WrongEventAccount,
  #[msg("Wrong event organizer")]
  WrongEventOrganizer,
  #[msg("Event if of token metadata does not match")]
  EventIdDoesNotMatch,
  #[msg("Wrong ticket owner")]
  WrongTicketOwner,
}
