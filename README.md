## 🔹 Overview
This is a **Solana-based Prediction Market Smart Contract** built using the **Anchor framework**. It enables users to predict outcomes of real-world events by trading "Yes" and "No" tokens.

---

## 🚀 Features
✅ **Decentralized Prediction Market** – Users can create their own market.
✅ **Deposite liquidity** – Users can deposite sol before start betting and once the liqudity amount reaches special amount they can start betting "Yes" and "No".
✅ **Betting** – Trade "Yes" and "No" tokens based on expected outcomes. Users can purchase "Yes" or "No" tokens based on their predictions, with token prices fluctuating dynamically according to probability. This probability is determined by the total number of tokens sold to users, ensuring a market-driven pricing mechanism.
✅ **Automated Settlement** – Resolves markets based on real-world data  
✅ **Switchboard Oracle Integration** – Fetches external data for outcome validation  

---

## 📜 Smart Contract Architecture
### 1️⃣ Gobal setting
```typescript
const tx = await program.methods.initialize({
      feeAuthority: feeAuthority,
      creatorFeeAmount: new BN(0.001 * 10 ** 9),
      liqudityUserFeeAmount: new BN(0.001 * 10 ** 9),
      bettingUserFeeAmount: new BN(0.001 * 10 ** 9),
      marketCount: new BN(0.1 * 10 ** 9),
      decimal: 9,
      feePercentage: 10,
    }).accounts({
      global,
      payer: owner.publicKey,
      systemProgram: SystemProgram.programId,
    }).signers([owner]).rpc();

### 2️⃣ Trading Mechanism
- Users can buy or sell **Yes/No** tokens  
- Prices are determined based on market supply and demand  

### 3️⃣ Market Resolution
- Switchboard Oracle fetches real-world data  
- The contract determines the winning outcome  
- Winning tokens are redeemable for rewards  

---

## 🛠 Installation & Setup
### 🔹 Prerequisites
- Rust & Solana CLI  
- Anchor framework  
- Node.js & npm  

### 🔹 Install Dependencies
\`\`\`bash
# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Install Anchor
cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked

# Install Node.js dependencies
npm install
\`\`\`

### 🔹 Build & Deploy
\`\`\`bash
anchor build
anchor deploy
\`\`\`

---

## 📝 Example Usage
### 1️⃣ Create a Market
\`\`\`rust
anchor test --filter create_market
\`\`\`
### 2️⃣ Place a Bet
\`\`\`rust
anchor test --filter place_bet
\`\`\`
### 3️⃣ Resolve Market
\`\`\`rust
anchor test --filter resolve_market
\`\`\`

---

## 📜 Smart Contract Functions
| Function | Description |
|----------|------------|
| \`create_market\` | Initializes a new prediction market |
| \`place_bet\` | Allows users to buy Yes/No tokens |
| \`resolve_market\` | Determines the winning outcome |
| \`withdraw_funds\` | Users claim winnings after market resolution |

---

## 📡 Switchboard Oracle Integration
This contract uses **Switchboard Oracles** to fetch real-time data for market resolutions.  
Example feed:  
\`\`\`rust
#[account(
    address = Pubkey::from_str(SOL_USDC_FEED).unwrap()
)]
pub feed_aggregator: AccountLoader<'info, AggregatorAccountData>,
\`\`\`

---

## 📜 License
This project is open-source under the **MIT License**.  

---

## 📩 Contact & Support
For questions or collaboration:  
📧 **Email:** [your-email@example.com](mailto:your-email@example.com)  
🐦 **Twitter:** [@yourhandle](https://twitter.com/yourhandle)  

---

### 🔥 **Join the Future of Decentralized Predictions on Solana!** 🔥  
