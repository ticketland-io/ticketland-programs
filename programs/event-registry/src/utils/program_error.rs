use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("Only Deployer")]
  OnlyDeployer,
  #[msg("Deposit token is not supported")]
  UnsupportedDepositToken,
  #[msg("Not enough deposit")]
  NotEnoughDeposit,
}
