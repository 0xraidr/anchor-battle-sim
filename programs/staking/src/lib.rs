use anchor_lang::prelude::*;

declare_id!("AkYu7jc18ErVQi7SJ8yPL9VMSusSXXXPY8nmfTWqtKSR");

#[program]
pub mod staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
