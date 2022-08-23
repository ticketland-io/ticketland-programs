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
use crate::{
  ticket_sale::{
    error::Error as TicketSaleError,
  }
};

#[derive(Debug)]
pub struct Error(pub secondary_market::utils::program_error::ErrorCode);

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl TryFrom<u32> for Error {
  type Error = ();

  fn try_from(value: u32) -> Result<Self, Self::Error> {
    match value - ERROR_CODE_OFFSET {
      0 => Ok(Error(secondary_market::utils::program_error::ErrorCode::WrongEventAccount)),
      1 => Ok(Error(secondary_market::utils::program_error::ErrorCode::OnlyEventOrganizer)),
      2 => Ok(Error(secondary_market::utils::program_error::ErrorCode::EventIdDoesNotMatch)),
      3 => Ok(Error(secondary_market::utils::program_error::ErrorCode::PriceCap)),
      4 => Ok(Error(secondary_market::utils::program_error::ErrorCode::WrongEventOrganizer)),
      5 => Ok(Error(secondary_market::utils::program_error::ErrorCode::WrongPurchaseToken)),
      6 => Ok(Error(secondary_market::utils::program_error::ErrorCode::WrongTicketSeller)),
      7 => Ok(Error(secondary_market::utils::program_error::ErrorCode::WrongTicketMetadata)),
      8 => Ok(Error(secondary_market::utils::program_error::ErrorCode::WrongSaleAccount)),
      9 => Ok(Error(secondary_market::utils::program_error::ErrorCode::WrongTicketNftState)),
      10 => Ok(Error(secondary_market::utils::program_error::ErrorCode::OnlyTicketOwner)),
      _ => Err(())
    }
  }
}

impl Error {
  pub fn assert_err(result: AnchorResult<()>, expected_error: secondary_market::utils::program_error::ErrorCode) {
    if let Err(LibError::ProgramError(ProgramErrorWithOrigin {program_error, ..})) = result {
      let code = Into::<u64>::into(program_error) as u32;
      let error = TryInto::<Error>::try_into(code).expect("no error found");
      
      assert_eq!(format!("{}", error), format!("{}", expected_error));
    } else {
      assert!(false, "expected error but none found")
    }
  }

  pub fn assert_ticket_sale_err(result: AnchorResult<()>, expected_error: ticket_sale::utils::program_error::ErrorCode) {
    if let Err(LibError::ProgramError(ProgramErrorWithOrigin {program_error, ..})) = result {
      let code = Into::<u64>::into(program_error) as u32;
      let error = TryInto::<TicketSaleError>::try_into(code).expect("no error found");
      
      assert_eq!(format!("{}", error), format!("{}", expected_error));
    } else {
      assert!(false, "expected error but none found")
    }
  }
}
