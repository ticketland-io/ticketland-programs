use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("Only deployer")]
  OnlyDeployer,
  #[msg("Too many currencies")]
  TooManyCurrencies,
  #[msg("Too many ticket types")]
  TooManyTicketTypes,
  #[msg("Deposit token is not supported")]
  UnsupportedDepositToken,
  #[msg("Not enough deposit")]
  NotEnoughDeposit,
  #[msg("Invalid Merkle proof")]
  InvalidMerkleProof,
  #[msg("Wrong ticket sale program state account")]
  WrongTicketSaleProgramStateAccount,
  #[msg("Only event organizer")]
  OnlyEventOrganizer,
  #[msg("Invalid ticket type index")]
  InvalidTicketTypeIndex,
}
