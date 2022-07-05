use anchor_lang::prelude::*;

declare_id!("TGfdMZj2HoSwdFR5zUAKr8H72XYJ85GQ7my5yZTHGKE");

#[program]
pub mod ticker_land_programs {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
