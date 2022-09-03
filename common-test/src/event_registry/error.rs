use std::{
  fmt,
  convert::TryFrom,
  convert::TryInto,
};
use anchor_lang::{
  prelude::Result as AnchorResult,
  error::{
    Error as LibError,
    ProgramErrorWithOrigin,
    ERROR_CODE_OFFSET,
  },
};

#[derive(Debug)]
pub struct Error(pub event_registry::utils::program_error::ErrorCode);

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl TryFrom<u32> for Error {
  type Error = ();

  fn try_from(value: u32) -> Result<Self, Self::Error> {
    match value - ERROR_CODE_OFFSET {
      0 => Ok(Error(event_registry::utils::program_error::ErrorCode::OnlyDeployer)),
      1 => Ok(Error(event_registry::utils::program_error::ErrorCode::TooManyCurrencies)),
      2 => Ok(Error(event_registry::utils::program_error::ErrorCode::TooManyTicketTypes)),
      3 => Ok(Error(event_registry::utils::program_error::ErrorCode::UnsupportedDepositToken)),
      4 => Ok(Error(event_registry::utils::program_error::ErrorCode::UnsupportedPurchaseToken)),
      5 => Ok(Error(event_registry::utils::program_error::ErrorCode::NotEnoughDeposit)),
      6 => Ok(Error(event_registry::utils::program_error::ErrorCode::InvalidMerkleProof)),
      7 => Ok(Error(event_registry::utils::program_error::ErrorCode::WrongTicketSaleProgramStateAccount)),
      11 => Ok(Error(event_registry::utils::program_error::ErrorCode::TicketSaleMustBeOwner)),
      _ => Err(())
    }
  }
}

impl Error {
  pub fn assert_err(result: AnchorResult<()>, expected_error: event_registry::utils::program_error::ErrorCode) {
    if let Err(LibError::ProgramError(ProgramErrorWithOrigin {program_error, ..})) = result {
      let code = Into::<u64>::into(program_error) as u32;
      let error = TryInto::<Error>::try_into(code).expect("no error found");
      
      assert_eq!(format!("{}", error), format!("{}", expected_error));
    } else {
      assert!(false, "expected error but none found")
    }
  }
}
