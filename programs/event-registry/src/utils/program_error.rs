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
  #[msg("Purchase token is not supported")]
  UnsupportedPurchaseToken,
  #[msg("Not enough deposit")]
  NotEnoughDeposit,
  #[msg("Invalid Merkle proof")]
  InvalidMerkleProof,
  #[msg("Wrong ticket sale program state account")]
  WrongTicketSaleProgramStateAccount,
  #[msg("Ticket sale must the the owner")]
  TicketSaleMustBeOwner,
  #[msg("Only URI update operator")]
  OnlyUriUpdateOperator,
}
