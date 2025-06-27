use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{
        close_account, transfer_checked, CloseAccount, Mint, Token, TokenAccount, TransferChecked,
    },
};

use crate::states::Escrow;

//Just have to refund the vault token into mint_a and close  the vault account
#[derive(Accounts)]
struct Cancel<'info>{
    #[account(mut)]
    pub initializer: Signer<'info>,

    pub mint_a :Account<'info, Mint>,

    #[account(mut, associated_token::mint=mint_a, associated_token::authority=initializer)]
    pub initializer_ata_a: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        has_one = initializer,
        has_one = mint_a,
        close = initializer,
        seeds=[b"state", escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
    )]
    escrow: Account<'info, Escrow>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow
    )]
    pub vault: Account<'info, TokenAccount>,
    associated_token_program: Program<'info, AssociatedToken>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

impl<'info> Cancel<'info>{
pub fn refund_and_close_vault(&mut self) ->Result<()>{
    let signer_seeds: &[&[&[u8]]] = &[&[
        b"state".as_ref(),
        &self.escrow.seed.to_le_bytes(),
        &[self.escrow.bump],
    ]];

    let cpi_account= token::TransferChecked{
        from : self.vault.to_account_info(),
        to : self.initializer_ata_a.to_account_info(),
        mint: self.mint_a.to_account_info(),
        authority: self.escrow.to_account_info(),

    };
   let context=  CpiContext::new( self.token_program.to_account_info(), cpi_account).with_signer(&signer_seeds);
   transfer_checked(context, self.escrow.initialize_amount, self.mint_a.decimals)?;


   let close_ctx =CpiContext::new(self.token_program.to_account_info(), 
        CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.initializer.to_account_info(),
            authority: self.escrow.to_account_info(),
        }
    ).with_signer(&signer_seeds);


    close_account(close_ctx)?;

    Ok(())

}

}