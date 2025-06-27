use anchor_lang::prelude::*;

declare_id!("7sESMPNrnBgSZm9bBJ5h7vCZ7KudnBnhaHvXWHpomEh3");

mod contexts;
mod states;
use contexts::*;
#[program]
pub mod anchor_escrow {
    use super::*;
pub fn initialize(ctx:Context<Initialize> , seeds:u64 , initialize_amount:u64 , taker_amount:u64)->Result<()>{

    ctx.accounts
            .initialize_escrow(seed, &ctx.bumps, initializer_amount, taker_amount)?;
        ctx.accounts.deposit(initializer_amount)?;
        Ok(())
};
pub fn cancel(ctx:Context<Cancel>)->Result<()>{
    ctx.accounts.refund_and_close_account();
    Ok(())
};


pub fn exchange(ctx:Context<Exchange>)->Result<()>{
ctx.accounts.deposit();
ctx.account.withdraw_and_close_vault();

    Ok(())
};
}

