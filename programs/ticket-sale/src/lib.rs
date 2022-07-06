use anchor_lang::prelude::*;

declare_id!("6banhWF9WKQk26NtgX6TqHNmAKgvX9aJmgaBvPmYCCK3");

#[program]
pub mod ticket_sale {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
