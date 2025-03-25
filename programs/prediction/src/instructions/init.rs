use crate::constants::GLOBAL_SEED;
use crate::events::GlobalInitialized;
use crate::states::global::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, payer = payer, space = 8 + Global::INIT_SPACE, seeds = [GLOBAL_SEED.as_bytes()], bump)]
    pub global: Account<'info, Global>,
    pub system_program: Program<'info, System>,
}

pub fn init(ctx: Context<Initialize>, params: GlobalParams) -> Result<()> {
    let global = &mut ctx.accounts.global;
    global.admin = ctx.accounts.payer.key();
    global.fee_authority = params.fee_authority;
    global.fee_percentage = params.fee_percentage;
    emit!(GlobalInitialized {
        global_id: global.key(),
        fee_recipient: global.fee_authority,
        fee_percentage: global.fee_percentage,
    });

    Ok(())
}
