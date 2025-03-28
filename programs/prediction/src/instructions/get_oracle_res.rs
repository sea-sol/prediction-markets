use crate::constants::{MARKET_SEED, SOL_USDC_FEED};
use crate::events::OracleResUpdated;
use crate::states::market::Market;
use anchor_lang::prelude::*;
use std::str::FromStr;
use switchboard_on_demand::on_demand::accounts::pull_feed::PullFeedAccountData;
use switchboard_solana::AggregatorAccountData;

#[derive(Accounts)]
pub struct GetOracleRes<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [MARKET_SEED.as_bytes(), user.key().as_ref()],
        bump
    )]
    /// CHECK: global fee authority is checked in constraint
    pub market: Box<Account<'info, Market>>,
    #[account(
        address = Pubkey::from_str(SOL_USDC_FEED).unwrap()
    )]
    pub feed_aggregator: AccountLoader<'info, AggregatorAccountData>,
    /// CHECK: via switchboard sdk
    pub feed: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn get_oracle_res(ctx: Context<GetOracleRes>) -> Result<()> {
    let feed = &ctx.accounts.feed_aggregator.load()?;
    let current_sol_price: f64 = feed.get_result()?.try_into()?;

    msg!("🎫Current SOL/USD price 🎫{}", current_sol_price);

    let feed_account = ctx.accounts.feed.data.borrow();
    let feed: std::cell::Ref<'_, PullFeedAccountData> =
        PullFeedAccountData::parse(feed_account).unwrap();
    msg!("🎫price 🎫 {:?}", feed.value());

    if ctx.accounts.market.quest <= feed.value().unwrap().try_into().unwrap() {
        ctx.accounts.market.update_result(true);
    } else {
        ctx.accounts.market.update_result(false);
    }
    msg!("🎫result 🎫 {:?}", ctx.accounts.market.result);

    emit!(OracleResUpdated {
        oracle_res: feed.value().unwrap().try_into().unwrap(),
    });
    Ok(())
}
