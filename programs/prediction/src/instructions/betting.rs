use crate::constants::{GLOBAL_SEED, MARKET_SEED};
use crate::states::{global::*, market::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Betting<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: creator is checked in constraint
    pub creator: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [MARKET_SEED.as_bytes(), creator.key().as_ref()],
        constraint = market.market_status == MarketStatus::Active,
        bump
    )]
    pub market: Account<'info, Market>,

    #[account(
        seeds = [GLOBAL_SEED.as_bytes()],
        bump
    )]
    pub global: Account<'info, Global>,

    pub system_program: Program<'info, System>,
}

impl Betting<'_> {
    pub fn betting(ctx: Context<Betting>, amount: u64) -> Result<()> {
        Ok(())
    }
}


