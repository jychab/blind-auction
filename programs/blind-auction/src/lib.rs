use anchor_lang::prelude::*;

declare_id!("Cy9dGwHUyKeSqfe4GUvwXB5V9zbzo3mk2tB6Jzgiu3q7");

#[program]
pub mod blind_auction {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
