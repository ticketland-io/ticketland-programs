use anchor_lang::{
  prelude::*,
  AccountsClose,
};
use crate::{
  account_data::{
    sell_listing_reservation::*,
  },
  utils::program_error::ErrorCode,
};

pub fn check<'info>(
  fill_listing_reservation: &AccountInfo<'info>,
  operator: &AccountInfo<'info>,
  ticket_buyer: &Signer<'info>,
) -> Result<()> {
  // if account exists then check if it has expired or the sender is the recipient
  if fill_listing_reservation.lamports() != 0 {
    // This will no fail because it lamports is no 0. It will also check that fill_listing_reservation account
    // is owned by the TicketSale program
    let fill_listing_reservation = Account::<SellListingReservation>::try_from(&fill_listing_reservation)?;
    
    if fill_listing_reservation.recipient == ticket_buyer.key() || Clock::get().unwrap().slot > fill_listing_reservation.valid_until {
      fill_listing_reservation.close(operator.clone())?;
    } else {
      return Err(ErrorCode::SellListingReserved.into())
    }
  }

  Ok(())
}
