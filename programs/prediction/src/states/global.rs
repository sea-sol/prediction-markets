use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct Global {
    pub admin: Pubkey,
    pub fee_authority: Pubkey,
    pub creator_fee_amount: u64,
    pub liqudity_user_fee_amount: u64,
    pub betting_user_fee_amount: u64,
    pub decimal: u8,
    pub market_count: u64,
    pub fee_percentage: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct GlobalParams {
    pub fee_authority: Pubkey,
    pub creator_fee_amount: u64,
    pub liqudity_user_fee_amount: u64,
    pub betting_user_fee_amount: u64,
    pub market_count: u64,
    pub decimal: u8,
    pub fee_percentage: u8,
}
