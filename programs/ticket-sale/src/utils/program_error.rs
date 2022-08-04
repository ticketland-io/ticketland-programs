use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("Only Deployer")]
  OnlyDeployer,
  #[msg("Not owner by this program")]
  NotOwnedByThisProgram,
  #[msg("Not event account")]
  WrongEventAccount,
  #[msg("Wrong event capacity")]
  WrongEventCapacityAccount,
  #[msg("Invalid Merkle proof")]
  InvalidProof,
  #[msg("Invalid sold out")]
  TicketSoldOut,
  #[msg("Seat not available")]
  SeatNotAvailable,
  #[msg("Expected different sale account")]
  UnexpectedSaleAccount,
  #[msg("Purchase token is not supported")]
  UnsupportedPurchaseToken,
  #[msg("Wrong event organizer")]
  WrongEventOrganizer,
  #[msg("Wrong treasury account")]
  WrongTreasuryAccount,
  #[msg("Sale has not started")]
  SaleNotStarted,
  #[msg("Wrong event organizer Sol treasury")]
  WrongSolTreasury,
  #[msg("Wrong Ticket NFT program")]
  WrongTicketNftProgram,
  #[msg("Wrong Ticket NFT program state account")]
  WrongTicketNftProgramStateAccount
}
