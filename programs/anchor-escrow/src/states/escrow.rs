use anchor_lang::prelude::*;

#[account]
struct Escrow{
    pub seed :u64,
    pub bump :u8,
    pub initializer :PubKey,
    pub mint_a :PubKey,
    pub mint_b:PubKey,
    pub initialize_amount :u64,
    pub taker_amount:u64
}

impl Space for Escrow{
    const INIT_SPACE:usize=8+8+1+32+32+32+8+8;
}

