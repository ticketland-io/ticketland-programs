use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("Not event account")]
  WrongEventAccount,
  #[msg("Wrong event organizer")]
  WrongEventOrganizer,
}
