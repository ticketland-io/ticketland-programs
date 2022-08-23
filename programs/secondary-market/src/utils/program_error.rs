use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("Not event account")]
  WrongEventAccount,
  #[msg("Only event organizer")]
  OnlyEventOrganizer,
  #[msg("Event if of token metadata does not match")]
  EventIdDoesNotMatch,
  #[msg("Wrong ticket owner")]
  WrongTicketOwner,
  #[msg("Price cap exceeded")]
  PriceCap,
  #[msg("Wrong ticket owner purchase token ata")]
  WrongTicketOwnerPurchaseTokenAta,
  #[msg("Wrong purchase token account")]
  WrongPurchaseToken,
  #[msg("Wrong ticket seller account")]
  WrongTicketSeller,
  #[msg("Wrong ticket metadata account")]
  WrongTicketMetadata,
  #[msg("Wrong sale account")]
  WrongSaleAccount,
  #[msg("Wrong ticket nft state")]
  WrongTicketNftState,
  #[msg("Wrong ticket owner")]
  OnlyTicketOwner,
}
