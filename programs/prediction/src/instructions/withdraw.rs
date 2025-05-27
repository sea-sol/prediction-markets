use crate::constants::GLOBAL_SEED;
use crate::errors::ContractError;
use crate::states::global::Global;
use crate::states::market::Market;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        constraint = admin.key() == global.admin @ ContractError::InvalidAdmin
    )]
    pub admin: Signer<'info>,

    /// CHECK: sol reciever
    pub reciever: AccountInfo<'info>,

    #[account(
        seeds = [GLOBAL_SEED.as_bytes()],
        bump
    )]
    pub global: Box<Account<'info, Global>>,

    #[account(mut)]
    /// CHECK: global fee authority is checked in constraint
    pub market: Box<Account<'info, Market>>,

    pub system_program: Program<'info, System>,
}

impl Withdraw<'_> {
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        // let market = &mut ctx.accounts.market;
        // let mint_authority_signer: [&[u8]; 3] =
        //     Market::get_signer(&market.bump, &market_id.as_bytes());
        // let mint_auth_signer_seeds = &[&mint_authority_signer[..]];
        msg!("market lamports {}", ctx.accounts.market.get_lamports());
        ctx.accounts.market.sub_lamports(amount).unwrap();
        ctx.accounts.reciever.add_lamports(amount).unwrap();
        Ok(())
    }
}
