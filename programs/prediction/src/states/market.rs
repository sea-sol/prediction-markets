use crate::constants::MARKET_SEED;
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]

pub struct Market {
    pub creator: Pubkey,
    pub feed: Pubkey,
    pub quest: u16,
    pub market_status: MarketStatus,
    pub result: bool,
    pub token_a: Pubkey,
    pub token_b: Pubkey,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub token_price_a: u64,
    pub token_price_b: u64,
    pub yes_amount: u16,
    pub no_amount: u16,
}

impl Market {
    pub fn get_signer<'a>(bump: &'a u8, user: &'a Pubkey) -> [&'a [u8]; 3] {
        [
            MARKET_SEED.as_bytes(),
            user.as_ref(),
            std::slice::from_ref(bump),
        ]
    }

    pub fn update_market_settings(
        &mut self,
        quest: u16,
        creator: Pubkey,
        feed: Pubkey,
        token_a: Pubkey,
        token_b: Pubkey,
        token_a_amount: u64,
        token_b_amount: u64,
        token_price_a: u64,
        token_price_b: u64,
    ) {
        self.creator = creator;
        self.quest = quest;
        self.feed = feed;
        self.token_a = token_a;
        self.token_b = token_b;
        self.token_a_amount = token_a_amount;
        self.token_b_amount = token_b_amount;
        self.token_price_a = token_price_a;
        self.token_price_b = token_price_b;
        self.yes_amount = 1;
        self.no_amount = 1;
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
    Active,
    Finished,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct MarketParams {
    pub quest: u16,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub token_price_a: u64,
    pub token_price_b: u64,
    pub name_a: Option<String>,
    pub name_b: Option<String>,
    pub symbol_a: Option<String>,
    pub symbol_b: Option<String>,
    pub url_a: Option<String>,
    pub url_b: Option<String>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct BettingParams {
    pub amount: u64,
    pub is_yes: bool,
}
