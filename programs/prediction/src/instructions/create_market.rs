use crate::constants::MARKET_SEED;
use crate::states::market::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct CreateMarket<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init, 
        payer = user, 
        space = 8 + Market::INIT_SPACE, 
        seeds = [MARKET_SEED.as_bytes()], 
        bump
    )]
    pub market: Account<'info, Market>,

    /// CHECK: via switchboard sdk
    pub feed: AccountInfo<'info>,

    #[account(mut)]
    pub token_a: Account<'info, Mint>,
    #[account(mut)]
    pub token_b: Account<'info, Mint>,

    #[account(mut)]
    pub pda_token_a_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pda_token_b_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_token_a_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_b_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    /// CHECK: token program account
    pub token_program: Program<'info, Token>,
    /// CHECK: associated token program account
    pub associated_token_program: UncheckedAccount<'info>,
    /// CHECK: token metadata program account
    pub token_metadata_program: UncheckedAccount<'info>,
    /// CHECK: rent account
    pub rent: UncheckedAccount<'info>,
}

pub fn create_market(ctx: Context<CreateMarket>, params: MarketParams) -> Result<()> {
    let market = &mut ctx.accounts.market;
    market.update_market_settings(
        params,
        ctx.accounts.user.key(),
        ctx.accounts.feed.key(),
        ctx.accounts.token_a.key(),
        ctx.accounts.token_b.key(),
    );

    Ok(())
}
