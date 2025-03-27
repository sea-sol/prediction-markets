use crate::constants::{GLOBAL_SEED, MARKET_SEED};
use crate::errors::ContractError;
use crate::states::{global::*, market::*};
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{Mint, TokenAccount};
use crate::utils::token_transfer;

#[derive(Accounts)]
pub struct Betting<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: creator is checked in constraint
    pub creator: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK reward token
    token_mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = market
    )]
    pub pda_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut, 
        associated_token::mint = token_mint,
        associated_token::authority = user.key()
    )]
    pub user_token_account: Box<Account<'info, TokenAccount>>,

    /// CHECK: global fee authority is checked in constraint
    #[account(
        mut,
        constraint = fee_authority.key() == global.fee_authority @ ContractError::InvalidFeeAuthority
    )]
    pub fee_authority: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [MARKET_SEED.as_bytes(), creator.key().as_ref()],
        constraint = market.market_status == MarketStatus::Active @ ContractError::MarketNotActive,
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
    pub fn betting(ctx: Context<Betting>, params: BettingParams) -> Result<()> {
        let market = &mut ctx.accounts.market;
        let global = &mut ctx.accounts.global;

        if params.is_yes {
            market.yes_amount += 1;
        } else {
            market.no_amount += 1;
        }

        // Transfer sol to market
        let transfer_market_instruction = solana_program::system_instruction::transfer(
            ctx.accounts.user.key,
            market.to_account_info().key,
            params.amount,
        );

        // Transfer token to market
        let binding = market.key();
        let mint_authority_signer: [&[u8]; 3] =
            Market::get_signer(&ctx.bumps.market, &binding);
        let mint_auth_signer_seeds = &[&mint_authority_signer[..]];

        token_transfer(
            ctx.accounts.pda_token_account.to_account_info(),
            ctx.accounts.user_token_account.to_account_info(),
            market.to_account_info(),
            ctx.accounts.token_mint.to_account_info(),
            mint_auth_signer_seeds,
            params.amount,
        )?;

        // Invoke the transfer instruction
        anchor_lang::solana_program::program::invoke_signed(
            &transfer_market_instruction,
            &[
                ctx.accounts.user.to_account_info(),
                market.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[],
        )?;

        // Transfer fee to fee authority and creator
        let fee_amount_to_auth = ctx
            .accounts
            .global
            .betting_user_fee_amount
            .checked_mul(ctx.accounts.global.fee_percentage as u64)
            .ok_or(ContractError::ArithmeticError)?
            .checked_div(100)
            .ok_or(ContractError::ArithmeticError)?;

        let transfer_auth_instruction = solana_program::system_instruction::transfer(
            ctx.accounts.user.key,
            ctx.accounts.fee_authority.key,
            fee_amount_to_auth,
        );

        // Invoke the transfer instruction
        anchor_lang::solana_program::program::invoke_signed(
            &transfer_auth_instruction,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.fee_authority.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[],
        )?;

        let transfer_creator_instruction = solana_program::system_instruction::transfer(
            ctx.accounts.user.key,
            ctx.accounts.creator.key,
            ctx.accounts
                .global
                .betting_user_fee_amount
                .checked_sub(fee_amount_to_auth)
                .unwrap(),
        );

        // Invoke the transfer instruction
        anchor_lang::solana_program::program::invoke_signed(
            &transfer_creator_instruction,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.creator.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[],
        )?;

        Ok(())
    }
}
