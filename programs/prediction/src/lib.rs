use anchor_lang::prelude::*;
pub mod constants;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod states;
pub mod utils;

use instructions::{
    betting::*, create_market::*, deposite_liquidity::*, get_oracle_res::*, init::*, token_mint::*,
};
use states::{
    global::GlobalParams,
    market::{BettingParams, MarketParams},
};

declare_id!("Da4udDYBTFUrdCjRkstWMMpxQdPi6pkq7CXicSUNZA2u");

#[program]
pub mod prediction {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, params: GlobalParams) -> Result<()> {
        init(ctx, params)
    }

    pub fn get_res(ctx: Context<GetOracleRes>) -> Result<()> {
        get_oracle_res(ctx)
    }

    pub fn init_market(ctx: Context<CreateMarket>, params: MarketParams) -> Result<()> {
        CreateMarket::create_market(ctx, params)
    }

    pub fn add_liquidity(ctx: Context<DepositLiquidity>, amount: u64) -> Result<()> {
        deposit_liquidity(ctx, amount)
    }

    pub fn create_bet(ctx: Context<Betting>, params: BettingParams) -> Result<()> {
        Betting::betting(ctx, params)
    }

    pub fn mint_token(ctx: Context<TokenMint>) -> Result<()> {
        TokenMint::token_mint(ctx)
    }
}
