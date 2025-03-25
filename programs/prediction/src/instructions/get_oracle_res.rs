use crate::constants::SOL_USDC_FEED;
use crate::errors::ContractError;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;
use std::str::FromStr;
use switchboard_solana::AggregatorAccountData;

#[derive(Accounts)]
pub struct GetOracleRes<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        address = Pubkey::from_str(SOL_USDC_FEED).unwrap()
    )]
    pub feed_aggregator: AccountLoader<'info, AggregatorAccountData>,
    pub system_program: Program<'info, System>,
}

pub fn get_oracle_res(ctx: Context<GetOracleRes>) -> Result<()> {
    let feed = &ctx.accounts.feed_aggregator.load()?;
    let current_sol_price: f64 = feed.get_result()?.try_into()?;

    msg!("Current SOL/USD price: {}", current_sol_price);

    // Check if the feed has been updated in the last 5 minutes (300 seconds)
    feed.check_staleness(Clock::get().unwrap().unix_timestamp, 300)
        .map_err(|_| error!(ContractError::StaleFeed))?;

    Ok(())
}
