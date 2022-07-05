use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("Only Deployer")]
  OnlyDeployer,
  #[msg("Too many currencies")]
  TooManyCurrencies,
  #[msg("Too many ticket types")]
  TooManyTicketTypes,
  #[msg("Deposit token is not supported")]
  UnsupportedDepositToken,
  #[msg("Not enough deposit")]
  NotEnoughDeposit,
}
