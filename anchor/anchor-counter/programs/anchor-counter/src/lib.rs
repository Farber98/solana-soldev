use anchor_lang::prelude::*;

declare_id!("8FrZ8GeiCfEFkrPvEq8gXxfWtKgsSgZTYcfdz1f1bDm8");

#[program]
pub mod anchor_counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
