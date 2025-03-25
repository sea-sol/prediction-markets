use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct Global {
    pub admin: Pubkey,
    pub fee_authority: Pubkey,
    pub fee_percentage: u16,
}
