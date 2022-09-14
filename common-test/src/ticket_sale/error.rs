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
pub struct Error(pub ticket_sale::utils::program_error::ErrorCode);

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl TryFrom<u32> for Error {
  type Error = ();

  fn try_from(value: u32) -> Result<Self, Self::Error> {
    match value - ERROR_CODE_OFFSET {
      0 => Ok(Error(ticket_sale::utils::program_error::ErrorCode::OnlyDeployer)),
      1 => Ok(Error(ticket_sale::utils::program_error::ErrorCode::NotOwnedByThisProgram)),
      2 => Ok(Error(ticket_sale::utils::program_error::ErrorCode::WrongEventAccount)),
      3 => Ok(Error(ticket_sale::utils::program_error::ErrorCode::WrongEventCapacityAccount)),
      4 => Ok(Error(ticket_sale::utils::program_error::ErrorCode::InvalidProof)),
      5 => Ok(Error(ticket_sale::utils::program_error::ErrorCode::TicketSoldOut)),
      6 => Ok(Error(ticket_sale::utils::program_error::ErrorCode::SeatNotAvailable)),
      7 => Ok(Error(ticket_sale::utils::program_error::ErrorCode::UnexpectedSaleAccount)),
      8 => Ok(Error(ticket_sale::utils::program_error::ErrorCode::UnsupportedPurchaseToken)),
      9 => Ok(Error(ticket_sale::utils::program_error::ErrorCode::WrongEventOrganizer)),
      10 => Ok(Error(ticket_sale::utils::program_error::ErrorCode::WrongTreasuryAccount)),
      11 => Ok(Error(ticket_sale::utils::program_error::ErrorCode::SaleNotStarted)),
      12 => Ok(Error(ticket_sale::utils::program_error::ErrorCode::SaleFinished)),
      13 => Ok(Error(ticket_sale::utils::program_error::ErrorCode::WrongTicketNftProgram)),
      14 => Ok(Error(ticket_sale::utils::program_error::ErrorCode::WrongTicketNftProgramStateAccount)),
      15 => Ok(Error(ticket_sale::utils::program_error::ErrorCode::InvalidTicketTypeIndex)),
      16 => Ok(Error(ticket_sale::utils::program_error::ErrorCode::SeatNotVerified)),
      _ => Err(())
    }
  }
}

impl Error {
  pub fn assert_err(result: AnchorResult<()>, expected_error: ticket_sale::utils::program_error::ErrorCode) {
    if let Err(LibError::ProgramError(ProgramErrorWithOrigin {program_error, ..})) = result {
      let code = Into::<u64>::into(program_error) as u32;
      let error = TryInto::<Error>::try_into(code).expect("no error found");
      
      assert_eq!(format!("{}", error), format!("{}", expected_error));
    } else {
      assert!(false, "expected error but none found")
    }
  }
}
