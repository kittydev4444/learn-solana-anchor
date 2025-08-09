use anchor_lang::prelude::*;

declare_id!("D4yNnwGMxsTs3ZLhJN5wXHD4Xp65KixVyNsxD51hjsio");

#[program]
pub mod hello_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
