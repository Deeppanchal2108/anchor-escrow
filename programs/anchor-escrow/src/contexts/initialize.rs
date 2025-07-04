use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount ,TransferChecked , transfer_checked},
};

use crate::states::Escrow;

#[derive(Accounts)]
#[initialize(seed:u64, initialize_amount:u64)]
pub struct Initialize<'info>{

    #[account(mut)]
    pub initializer:Signer<'info>,

    pub mint_a:Account<'info,Mint>,
    pub mint_b:Account<'info,Mint>,

    #[account(
        mut, 
        constraint=initializer_ata_a.amount>= initialize_amount,
        associated_token::mint=mint_a,
        associated_token::authority=initializer
    )]
    pub initializer_ata_a:Account<'info,TokenAccount>,

    #[account(
        init_if_needed,
        payer = initializer,
        space = Escrow::INIT_SPACE,
        seeds = [b"state".as_ref(), &seed.to_le_bytes()],
        bump
    )]
    pub escrow : Account<'info, Escrow>,

    #[account(
        init_if_needed,
        payer= initializer,
        associated_token::mint=mint_a,
        associated_token::authority=escrow
    )]
    pub vault :Account<'info, TokenAccount>,

    pub associated_token_program :Program<'info, AssociatedToken>,   
    pub token_program :Program<'info, Token>,
    pub system_program:Program<'info, System>,

}

impl<'info> Initialize<'info>{

    pub fn initialize_escrow(&mut self , 
        initialize_amount:u64, 
        seed:u64,
        taker_amount:u64,
        bumps: &InitializeBumps

    ) -> Result<()> {
        self.escrow.set_inner(
            Escrow {
                seed,
                bump: bumps.escrow,
                initializer: self.initializer.key(),
                mint_a: self.mint_a.key(),
                mint_b: self.mint_b.key(),
                initialize_amount,
                taker_amount,
            }
        );

        Ok(())
    }

    pub fn deposit(&mut self , initialize_amount:u64)->Result<()>{
        
        let cpi_account= token::TransferChecked{
            from : self.initializer_ata_a.to_account_info(),
            to : self.vault.to_account_info(),
            mint: self.mint_a.to_account_info(),
            authority: self.initializer.to_account_info(),


        };
       let context=  CpiContext::new( self.token_program.to_account_info(), cpi_account);
       transfer_checked(context, initialize_amount, self.mint_a.decimals)?;

        Ok(())
    }
}