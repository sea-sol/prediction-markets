use crate::constants::SOL_USDC_FEED;
use crate::events::OracleResUpdated;
use anchor_lang::prelude::*;
use std::str::FromStr;
use switchboard_on_demand::on_demand::accounts::pull_feed::PullFeedAccountData;
use switchboard_solana::AggregatorAccountData;

#[derive(Accounts)]
pub struct GetOracleRes<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
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
    emit!(OracleResUpdated {
        oracle_res: feed.value().unwrap().try_into().unwrap(),
    });

    Ok(())
}
