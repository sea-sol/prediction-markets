use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]

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

impl Market {
    pub fn update_market_settings(
        &mut self,
        params: MarketParams,
        creator: Pubkey,
        feed: Pubkey,
        token_a: Pubkey,
        token_b: Pubkey,
    ) {
        self.creator = creator;
        self.quest = params.quest;
        self.feed = feed;
        self.token_a = token_a;
        self.token_b = token_b;
        self.token_a_amount = params.token_a_amount;
        self.token_b_amount = params.token_b_amount;
    }

    pub fn update_market_status(&mut self, market_status: MarketStatus) {
        self.market_status = market_status;
    }

    pub fn update_result(&mut self, result: bool) {
        self.result = result;
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace, Debug, PartialEq)]
pub enum MarketStatus {
    Prepare,
    NoActive,
    Active,
    Finished,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub struct MarketParams {
    pub quest: u64,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
}
