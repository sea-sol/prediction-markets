use anchor_lang::prelude::*;

#[account]
pub struct Market {
    pub creator: Pubkey,
    pub feed: Pubkey,
    pub quest: u64,
    pub market_status: MarketStatus,
    pub result: bool,
    pub token_a: Pubkey,
    pub token_b: Pubkey,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace, Debug, PartialEq)]
pub enum MarketStatus {
    Prepare,
    NoActive,
    Active,
    Finished,
}
