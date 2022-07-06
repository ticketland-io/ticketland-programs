use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommonError {
  #[error("Invalid Proof")]
  InvalidProof,
}
