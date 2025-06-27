// WHAT IS THIS EXCHANGE FLOW DOING?
// This code enables the taker (second user) to:
//  -> Send mint_b tokens to the initializer

//  -> Receive mint_a tokens from the escrow vault

//  -> Close the vault (returning rent to the initializer)

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount, TransferChecked, transfer_checked , CloseAccount, close_account},  

};

use crate::states::Escrow;

#[derive(Accounts)]
struct Exchange<'info>{
    #[account(mut)]
    pub taker :Signer<'info>,

    #[account(mut)]
    pub initializer: SystemAccount<'info>,

    pub mint_a: Account<'info, Mint>,
    pub mint_b: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint=mint_a,
        associated_token::authority=taker
    )]
    pub taker_ata_a: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint=mint_b,
        associated_token::authority=taker
    )]
    pub taker_ata_b: Box<Account<'info,TokenAccount>>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = initializer
    )]
    pub initializer_ata_b: Box<Account<'info, TokenAccount>>,

    #[account(
        mut ,
        close=initializer,
        has_one=mint_b,
        constraint= taker_ata_b.amount >= escrow.taker_amount,
        seeds = [b"state".as_ref(), &escrow.seed.to_le_bytes()],
        bump = escrow.bump

    )]
    pub escrow: Box<Account<'info, Escrow>>,


    #[account(mut,
    associated_token::mint=mint_a,
associated_token::authority=escrow)]
pub vault :Box<Account<'info, TokenAccount>>,

    pub associated_token :Program<'info , AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,


}

impl<'info> Exchange<'info>{


    pub fn deposit(&mut self)->Result<()>{

            
        let cpi_account= token::TransferChecked{
            from : self.taker_ata_b.to_account_info(),
            to : self.initializer_ata_b.to_account_info(),
            mint: self.mint_b.to_account_info(),
            authority: self.taker.to_account_info(),

        };
       let context=  CpiContext::new( self.token_program.to_account_info(), cpi_account);
       transfer_checked(context, self.escrow.taker_amount, self.mint_b.decimals)?;

        Ok(())
    }

    pub fn withdraw_and_close_vault(&mut self) -> Result<()> {
        let signer_seeds :&[&[&[u8]]]=&[&[
            b"state".as_ref(),
            &self.escrow.seed.to_le_bytes(),
            &[self.escrow.bump]
        ]];


             
        let cpi_account= token::TransferChecked{
            from : self.vault.to_account_info(),
            to : self.taker_ata_a.to_account_info(),
            mint: self.mint_a.to_account_info(),
            authority: self.escrow.to_account_info(),

        };
       let context=  CpiContext::new( self.token_program.to_account_info(), cpi_account).with_signer(signer_seeds);
       transfer_checked(context, self.escrow.initialize_amount, self.mint_a.decimals)?;


       let close_ctx =CpiContext::new(self.token_program.to_account_info(), 
            CloseAccount {
                account: self.vault.to_account_info(),
                destination: self.initializer.to_account_info(),
                authority: self.escrow.to_account_info(),
            }
        ).with_signer(signer_seeds);

        close_account(close_ctx)?;

        Ok(())

    }

}