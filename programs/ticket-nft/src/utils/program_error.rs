use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("Only Deployer")]
  OnlyDeployer,
  #[msg("Not ticket sale program")]
  NotTicketSaleProgram,
}
