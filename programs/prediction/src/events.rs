use crate::states::market::MarketStatus;
use anchor_lang::prelude::*;
#[event]
pub struct GlobalInitialized {
    pub global_id: Pubkey,
    pub fee_recipient: Pubkey,
    pub creator_fee_amount: u64,
    pub liqudity_user_fee_amount: u64,
    pub betting_user_fee_amount: u64,
    pub market_count: u64,
    pub decimal: u8,
}

#[event]
pub struct OracleResUpdated {
    pub oracle_res: f64,
}

#[event]
pub struct MarketCreated {
    pub market_id: Pubkey,
    pub quest: u16,
    pub creator: Pubkey,
    pub feed: Pubkey,
    pub token_a: Pubkey,
    pub token_b: Pubkey,
    pub market_status: MarketStatus,
}
