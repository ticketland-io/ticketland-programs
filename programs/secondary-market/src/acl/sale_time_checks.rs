use anchor_lang::{
  prelude::*,
};
use common::{
  account_data::{
    serialization::deser,
  },
};

use ticket_sale::{
  account_data::{
    sale::*,
  },
  utils::program_error::ErrorCode as TicketSaleErrorCode
};


pub fn check<'info>(sale: &AccountInfo<'info>) -> Result<()> {
  let sale: Sale = deser(sale.clone())?;
  require!(Clock::get().unwrap().unix_timestamp <= sale.ticket_type.sale_end_time, TicketSaleErrorCode::SaleFinished);
  
  Ok(())
}
