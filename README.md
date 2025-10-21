# Prediction Market Smart Contract

A decentralized prediction market platform built on Solana using the Anchor framework. This smart contract enables users to create and participate in prediction markets by trading outcome tokens based on real-world events.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Smart Contract Architecture](#smart-contract-architecture)
  - [Initialization](#initialization)
  - [Market Creation](#market-creation)
  - [Liquidity Deposit](#liquidity-deposit)
  - [Betting Mechanism](#betting-mechanism)
  - [Oracle Resolution](#oracle-resolution)
- [Installation and Setup](#installation-and-setup)
- [Usage Guide](#usage-guide)
- [Technical Stack](#technical-stack)

## Overview

This Solana-based prediction market smart contract leverages the Anchor framework to provide a trustless and decentralized platform for event outcome speculation. Users can create prediction markets, provide liquidity, and trade "Yes" and "No" tokens representing different outcomes. Market resolution is automated through Switchboard Oracle integration, ensuring accurate and tamper-proof results.

## Features

### Decentralized Market Creation
Users can autonomously create prediction markets for any real-world event.

### Liquidity Provision
Participants can deposit SOL tokens to provide initial liquidity. Markets become active once the liquidity threshold is reached.

### Dynamic Token Trading
Users can purchase "Yes" or "No" outcome tokens, with prices dynamically adjusting based on market probability. The probability mechanism is determined by the total token distribution, creating a self-balancing market-driven pricing system.

### Automated Settlement
Markets are automatically resolved using real-world data through oracle integration.

### Switchboard Oracle Integration
External data feeds validate outcomes, ensuring accurate and verifiable market resolution.

## Smart Contract Architecture

### Initialization

The contract must be initialized with global parameters before any markets can be created.

**TypeScript Example:**
```typescript
const tx = await program.methods.initialize({
  feeAuthority: feeAuthority,
  creatorFeeAmount: new BN(0.001 * 10 ** 9),
  liquidityUserFeeAmount: new BN(0.001 * 10 ** 9),
  bettingUserFeeAmount: new BN(0.001 * 10 ** 9),
  marketCount: new BN(0.1 * 10 ** 9),
  decimal: 9,
  feePercentage: 10,
}).accounts({
  global,
  payer: owner.publicKey,
  systemProgram: SystemProgram.programId,
}).signers([owner]).rpc();
```

**Rust Account Structure:**
```rust
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init, 
        payer = payer, 
        space = 8 + Global::INIT_SPACE, 
        seeds = [GLOBAL_SEED.as_bytes()], 
        bump
    )]
    pub global: Account<'info, Global>,
    pub system_program: Program<'info, System>,
}
```

### Market Creation

Market creation involves setting up a Program Derived Address (PDA), configuring initial parameters for outcome tokens, and minting the tokens.

**TypeScript Example:**
```typescript
const tx = await program.methods.initMarket({
  quest: 190,
  tokenAmount: new BN(tokenAAmount),
  tokenPrice: new BN(0.00005 * 10 ** 9),
  nameA: "tokenA",
  nameB: "tokenB",
  symbolA: "tokenA",
  symbolB: "tokenB",
  urlA: "https://tokenA.com",
  urlB: "https://tokenB.com",
}).accounts({
  user: owner.publicKey,
  feeAuthority: feeAuthority,
  market,
  // ...additional accounts
}).transaction();
```

**Dynamic Pricing Algorithm:**

The contract implements a dynamic pricing mechanism that adjusts token prices based on trading activity:

```rust
pub fn set_token_price(&mut self, sell_token_amount: u64, is_yes: bool) -> Result<()> {
    if is_yes {
        self.token_a_amount = self.token_a_amount - sell_token_amount;
        self.token_price_b = self.token_price_b + sell_token_amount;
    } else {
        self.token_b_amount = self.token_b_amount - sell_token_amount;
        self.token_price_a = self.token_price_a + sell_token_amount;
    }

    self.token_price_a = self
        .total_reserve
        .checked_mul(self.token_a_amount + self.token_b_amount)
        .ok_or(ContractError::ArithmeticError)?
        .checked_div(self.token_a_amount)
        .ok_or(ContractError::ArithmeticError)?;
    self.token_price_b = self
        .total_reserve
        .checked_mul(self.token_a_amount + self.token_b_amount)
        .ok_or(ContractError::ArithmeticError)?
        .checked_div(self.token_b_amount)
        .ok_or(ContractError::ArithmeticError)?;
    Ok(())
}
```

### Liquidity Deposit

Users provide liquidity to newly created markets by depositing SOL tokens.

**TypeScript Example:**
```typescript
let tx = await program.methods.addLiquidity(new BN(0.1 * 10 ** 9))
  .accounts({
    user: owner.publicKey,
    creator: owner.publicKey,
    feeAuthority: feeAuthority,
    market,
    global,
    systemProgram: SystemProgram.programId,
  }).transaction();
```

**Market Activation:**

Once the deposited liquidity reaches the threshold, the market status changes to active, enabling betting:

```rust
let market_balance = ctx.accounts.market.get_lamports();
if market_balance >= ctx.accounts.global.market_count {
  ctx.accounts.market.market_status = MarketStatus::Active;
}
```

### Betting Mechanism

Users can purchase "Yes" or "No" tokens representing their prediction on the market outcome.

**TypeScript Example:**
```typescript
const tx = await program.methods.createBet({
  amount: new BN(10000),
  isYes: true,
}).accounts({
  user: owner.publicKey,
  creator: owner.publicKey,
  tokenMint: tokenA,
  pdaTokenAccount: pdaTokenAAccount,
  userTokenAccount: userTokenAAccount,
  feeAuthority: feeAuthority,
  market,
  global,
  tokenProgram: TOKEN_PROGRAM_ID,
  associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
  systemProgram: SystemProgram.programId,
}).transaction();
```

**Price Dynamics:**

Token prices adjust dynamically based on trading volume and token distribution, maintaining market equilibrium.

![Token Price Dynamics - Chart 1](https://github.com/user-attachments/assets/43a27f40-ab88-48d1-a8ec-52a3a276c840)

![Token Price Dynamics - Chart 2](https://github.com/user-attachments/assets/b183f8bd-7cc4-403a-88c7-941cd9d94b8b)

### Oracle Resolution

Market outcomes are resolved automatically using Switchboard Oracle price feeds.

**Rust Implementation:**
```rust
let feed = &ctx.accounts.feed_aggregator.load()?;
let current_sol_price: f64 = feed.get_result()?.try_into()?;

msg!("Current SOL/USD price: {}", current_sol_price);

let feed_account = ctx.accounts.feed.data.borrow();
let feed: std::cell::Ref<'_, PullFeedAccountData> =
  PullFeedAccountData::parse(feed_account).unwrap();
msg!("Oracle price: {:?}", feed.value());

if ctx.accounts.market.quest <= feed.value().unwrap().try_into().unwrap() {
  ctx.accounts.market.update_result(true);
} else {
  ctx.accounts.market.update_result(false);
}
```

## Installation and Setup

### Prerequisites

- **Rust** (latest stable version)
- **Solana CLI** (v1.14 or higher)
- **Anchor Framework** (v0.28 or higher)
- **Node.js** (v16 or higher) and npm/yarn

### Installation Steps

1. **Install Solana CLI:**
   ```bash
   sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
   ```

2. **Install Anchor Framework:**
   ```bash
   cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked
   ```

3. **Install Node.js Dependencies:**
   ```bash
   npm install
   # or
   yarn install
   ```

### Build and Deploy

1. **Build the Smart Contract:**
   ```bash
   anchor build
   ```

2. **Deploy to Solana:**
   ```bash
   anchor deploy
   ```

## Usage Guide

### Configuration

Before deployment, update the seed constants in both TypeScript and Rust files:

**TypeScript (`tests/const.ts`):**
```typescript
export const GLOBAL_SEED = "global_7";
export const MARKET_SEED = "market_7";
export const MINT_SEED_A = "mint_a_7";
export const MINT_SEED_B = "mint_b_7";
```

**Rust (`programs/prediction/src/constants.rs`):**
```rust
pub const GLOBAL_SEED: &'static str = "global_7";
pub const MARKET_SEED: &'static str = "market_7";
pub const MINT_SEED_A: &'static str = "mint_a_7";
pub const MINT_SEED_B: &'static str = "mint_b_7";
```

### Running Tests

Execute the test suite to verify contract functionality:
```bash
anchor test
```

## Technical Stack

- **Blockchain:** Solana
- **Framework:** Anchor
- **Programming Languages:** Rust, TypeScript
- **Oracle:** Switchboard
- **Token Standard:** SPL Token

---

**License:** See LICENSE file for details.

**Contributing:** Contributions are welcome. Please submit pull requests or open issues for any improvements or bug reports.
