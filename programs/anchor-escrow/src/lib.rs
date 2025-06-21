use anchor_lang::prelude::*;

declare_id!("7sESMPNrnBgSZm9bBJ5h7vCZ7KudnBnhaHvXWHpomEh3");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
