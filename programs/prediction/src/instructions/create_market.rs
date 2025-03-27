use crate::constants::{
    MARKET_SEED, GLOBAL_SEED, MINT_SEED_A, MINT_SEED_B,
};
use crate::states::{
    market::*, global::*,
};
use anchor_lang::{
    prelude::*, solana_program,
};
use crate::events::MarketCreated;
use anchor_spl::{
    metadata::{
        create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3, 
    },
    token::{
        Mint, TokenAccount, mint_to, MintTo, Token,
    },
};
use crate::errors::ContractError;

#[derive(Accounts)]
#[instruction(params: MarketParams)]
pub struct CreateMarket<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    /// CHECK: global fee authority is checked in constraint
    #[account(
        mut,
        constraint = fee_authority.key() == global.fee_authority @ ContractError::InvalidFeeAuthority
    )]
    pub fee_authority: AccountInfo<'info>,

    #[account(
        init, 
        payer = user, 
        space = 8 + Market::INIT_SPACE, 
        seeds = [MARKET_SEED.as_bytes(), user.key().as_ref()], 
        bump
    )]
    pub market: Box<Account<'info, Market>>,
    
    #[account(
        seeds = [GLOBAL_SEED.as_bytes()],
        bump
    )]
    pub global: Box<Account<'info, Global>>,
    /// CHECK: via switchboard sdk
    pub feed: AccountInfo<'info>,
    
    #[account(mut)]
    ///CHECK: Using seed to validate metadata account
    metadata: UncheckedAccount<'info>,
    
    #[account(
        init,
        seeds = [MINT_SEED_A.as_bytes(), market.key().as_ref()],
        bump,
        payer = user,
        mint::decimals = global.decimal,
        mint::authority = market,
    )]
    token_mint_a: Box<Account<'info, Mint>>,
    #[account(
        init,
        seeds = [MINT_SEED_B.as_bytes(), market.key().as_ref()],
        bump,
        payer = user,
        mint::decimals = global.decimal,
        mint::authority = market
    )]
    token_mint_b: Box<Account<'info, Mint>>,
    
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_mint_a,
        associated_token::authority = market
    )]
    pub pda_token_a_account: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_mint_b,
        associated_token::authority = market
    )]
    pub pda_token_b_account: Box<Account<'info, TokenAccount>>,

    // #[account(
    //     mut, 
    //     associated_token::mint = token_mint_a,
    //     associated_token::authority = user.key()
    // )]
    // pub user_token_a_account: Box<Account<'info, TokenAccount>>,
    // #[account(
    //     mut, 
    //     associated_token::mint = token_mint_b,
    //     associated_token::authority = user.key()
    // )]
    // pub user_token_b_account: Box<Account<'info, TokenAccount>>,
    
    pub token_program: Program<'info, Token>,
    /// CHECK: associated token program account
    pub associated_token_program: UncheckedAccount<'info>,
    /// CHECK: token metadata program account
    pub token_metadata_program: UncheckedAccount<'info>,
    /// CHECK: rent account
    pub rent: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl CreateMarket<'_> {
    pub fn create_market(ctx: Context<CreateMarket>, params: MarketParams) -> Result<()> {
        let decimal_multiplier = 10u64.pow(ctx.accounts.global.decimal as u32);
        let token_a_amount = params
            .token_a_amount
            .checked_mul(decimal_multiplier)
            .ok_or(ContractError::ArithmeticError)?;
        let token_b_amount = params
            .token_b_amount
            .checked_mul(decimal_multiplier)
            .ok_or(ContractError::ArithmeticError)?;

        // update market settings
        ctx.accounts.market.update_market_settings(
            params.quest,
            ctx.accounts.user.key(),
            ctx.accounts.feed.key(),
            ctx.accounts.token_mint_a.key(),
            ctx.accounts.token_mint_b.key(),
            token_a_amount,
            token_b_amount,
            params.token_price_a,
            params.token_price_b,
        );

        let binding = ctx.accounts.market.key();
        let mint_authority_signer: [&[u8]; 3] =
            Market::get_signer(&ctx.bumps.market, &binding);
        let mint_auth_signer_seeds = &[&mint_authority_signer[..]];
        
        // initialize metadata and mint "Yes" token to market
        ctx.accounts.intialize_meta(
            params.name_a,
            true,
            params.symbol_a,
            params.url_a,
            mint_auth_signer_seeds,
        )?;

        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    authority: ctx.accounts.market.to_account_info().clone(),
                    to: ctx.accounts.pda_token_a_account.to_account_info(),
                    mint: ctx.accounts.token_mint_a.to_account_info().clone(),
                },
                mint_auth_signer_seeds,
            ),
            token_a_amount,
        )?;

        // initialize metadata and mint "No" token to market    
        ctx.accounts.intialize_meta(
            params.name_b,
            false,
            params.symbol_b,
            params.url_b,
            mint_auth_signer_seeds,
        )?;     

        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    authority: ctx.accounts.market.to_account_info().clone(),
                    to: ctx.accounts.pda_token_b_account.to_account_info(),
                    mint: ctx.accounts.token_mint_b.to_account_info().clone(),
                },
                mint_auth_signer_seeds,
            ),
            token_b_amount,
        )?;

        // Transfer creator fee to fee authority
        let transfer_instruction =
        solana_program::system_instruction::transfer(ctx.accounts.user.key, ctx.accounts.fee_authority.key, ctx.accounts.global.creator_fee_amount);

        // Invoke the transfer instruction
        anchor_lang::solana_program::program::invoke_signed(
            &transfer_instruction,
            &[ctx.accounts.user.to_account_info(), ctx.accounts.fee_authority.to_account_info(), ctx.accounts.system_program.to_account_info()],
            &[],
        )?;

        ctx.accounts.market.update_market_status(MarketStatus::Prepare);
        // // Transfer token a and token b from user to market
        // let cpi_ctx_a: CpiContext<_> = CpiContext::new(
        //     ctx.accounts.token_program.to_account_info(),
        //     Transfer {
        //         from: ctx.accounts.user_token_a_account.to_account_info(),
        //         authority: ctx.accounts.user.to_account_info(),
        //         to: ctx.accounts.pda_token_a_account.to_account_info(),
        //     },
        // );
        // transfer(cpi_ctx_a, params.token_a_amount)?;

        // let cpi_ctx_b: CpiContext<_> = CpiContext::new(
        //     ctx.accounts.token_program.to_account_info(),
        //     Transfer {
        //         from: ctx.accounts.user_token_b_account.to_account_info(),
        //         authority: ctx.accounts.user.to_account_info(),
        //         to: ctx.accounts.pda_token_b_account.to_account_info(),
        //     },
        // );
        // transfer(cpi_ctx_b, params.token_b_amount)?;

        emit!(MarketCreated {
            market_id: ctx.accounts.market.key(),
            quest: ctx.accounts.market.quest,
            creator: ctx.accounts.user.key(),
            feed: ctx.accounts.feed.key(),
            token_a: ctx.accounts.token_mint_a.key(),
            token_b: ctx.accounts.token_mint_b.key(),
            market_status: ctx.accounts.market.market_status,
            token_a_amount: ctx.accounts.market.token_a_amount,
            token_b_amount: ctx.accounts.market.token_b_amount,
            token_price_a: ctx.accounts.market.token_price_a,
            token_price_b: ctx.accounts.market.token_price_b,
        }); 

        Ok(())
    }
    pub fn intialize_meta(
        &mut self,
        name: Option<String>,
        is_token_a: bool,
        symbol: Option<String>,
        uri: Option<String>,
        mint_auth_signer_seeds: &[&[&[u8]]; 1],
    ) -> Result<()> {
        let mint_info: AccountInfo<'_>;
        if is_token_a {
            mint_info = self.token_mint_a.to_account_info();
        } else {
            mint_info = self.token_mint_b.to_account_info();
        }
        
        let mint_authority_info = self.market.to_account_info();
        let metadata_info = self.metadata.to_account_info();
    
        let mut para_name: String = "".to_string();
        let mut para_symbol: String = "".to_string();
        let mut para_uri: String = "".to_string();
    
        if let Some(name) = name {
            para_name = name.to_string();
        }
        if let Some(symbol) = symbol {
            para_symbol = symbol.to_string();
        }
        if let Some(uri) = uri {
            para_uri = uri.to_string();
        }
    
        let token_data: DataV2 = DataV2 {
            name: para_name.clone(),
            symbol: para_symbol.clone(),
            uri: para_uri.clone(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };
        let metadata_ctx = CpiContext::new_with_signer(
            self.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                payer: self.user.to_account_info(),
                mint: mint_info.clone(),
                metadata: metadata_info.clone(),
                update_authority: mint_authority_info.clone(),
                mint_authority: mint_authority_info.clone(),
                system_program: self.system_program.to_account_info(),
                rent: self.rent.to_account_info(),
            },
            mint_auth_signer_seeds,
        );
        create_metadata_accounts_v3(metadata_ctx, token_data, false, true, None)?;
        msg!("CollectionToken::intialize_meta: done");
        Ok(())
    }
}
