use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("Only Deployer")]
  OnlyDeployer,
  #[msg("Not owner by this program")]
  NotOwnedByThisProgram,
  #[msg("Not event account")]
  WrongEventAccount,
  #[msg("Not event capacity account")]
  WrongEventCapacityAccount,
  #[msg("Invalid Merkle proof")]
  InvalidProof,
  #[msg("Invalid sold out")]
  TicketSoldOut,
  #[msg("Seat not available")]
  SeatNotAvailable,
  #[msg("Expected fixed price sale")]
  ExpectedFixedPriceSaleAccount,
  #[msg("Purchase token is not supported")]
  UnsupportedPurchaseToken,
  #[msg("Wrong event organizer")]
  WrongEventOrganizer,
  #[msg("Wrong treasury account")]
  WrongTreasuryAccount,
}
