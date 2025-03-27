import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Prediction } from "../target/types/prediction";
import { PublicKey, SystemProgram, Keypair, Transaction, Connection, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { GLOBAL_SEED, PREDICTION_ID, SOL_USDC_FEED, MARKET_SEED, TOKEN_METADATA_PROGRAM_ID, tokenA, tokenB, tokenAAmount, tokenBAmount } from "./const";
import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync, getMint } from "@solana/spl-token";
import { getOrCreateATAInstruction, getAssociatedTokenAccount } from "./utils";
import BN from "bn.js";
let global: PublicKey;
let owner: Keypair;
let provider: anchor.AnchorProvider;
const connection = new Connection("https://api.devnet.solana.com");

describe("prediction", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  owner = anchor.Wallet.local().payer;
  provider = anchor.AnchorProvider.env();
  const program = anchor.workspace.Prediction as Program<Prediction>;
  before(async () => {
    const OracleEvent = await program.addEventListener("OracleResUpdated", (event, slot, signature) => {
      console.log("👻OracleResUpdated 👻", Number(event.oracleRes));
    });

    const GlobalEvent = await program.addEventListener("GlobalInitialized", (event, slot, signature) => {
      console.log("👻GlobalInitilized  👻", event);
    });

    const MarketEvent = await program.addEventListener("MarketCreated", (event, slot, signature) => {
      console.log("👻MarketCreated  👻", event);
    });

    global = PublicKey.findProgramAddressSync([Buffer.from(GLOBAL_SEED)], PREDICTION_ID)[0];
  });
  
  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize({
      feeAuthority: owner.publicKey,
      creatorFeeAmount: new BN(0.001 * 10 ** 9),
      liqudityUserFeeAmount: new BN(0.001 * 10 ** 9),
      bettingUserFeeAmount: new BN(0.001 * 10 ** 9),
      marketCount: new BN(0.1 * 10 ** 9),
    }).accounts({
      global,
      payer: owner.publicKey,
      systemProgram: SystemProgram.programId,
    }).signers([owner]).rpc();
    console.log("🤖Your transaction signature 🤖", tx);
  });

  it("Get oracle res", async () => {
    const createTx = new Transaction()

    const tx = await program.methods.getRes().accounts({
      user: owner.publicKey,
      feedAggregator: new PublicKey(SOL_USDC_FEED),
      feed: new PublicKey("5mXfTYitRFsWPhdJfp2fc8N6hK8cw6NB5jAYpronQasj"),
      systemProgram: SystemProgram.programId,
    }).signers([owner]).rpc();

    console.log("🤖Your transaction signature  🤖", tx);
  });

  it("Create market", async () => {
    const market = PublicKey.findProgramAddressSync([Buffer.from(MARKET_SEED), owner.publicKey.toBuffer()], PREDICTION_ID)[0];
    let userTokenAAccount = getAssociatedTokenAddressSync(tokenA, owner.publicKey);
    let userTokenBAccount = getAssociatedTokenAddressSync(tokenB, owner.publicKey);
    let pdaTokenAAccount = await getAssociatedTokenAccount(market, tokenA);
    let pdaTokenBAccount = await getAssociatedTokenAccount(market, tokenB);

    // Create market //////////////////////////////////////////////////////////////////////////////////////////////////////////////
    const tx = await program.methods.initMarket(190, tokenA, tokenB).accounts({
      user: owner.publicKey,
      market,
      feed: new PublicKey("5mXfTYitRFsWPhdJfp2fc8N6hK8cw6NB5jAYpronQasj"),
      systemProgram: SystemProgram.programId,
      feeAuthority: owner.publicKey,
      global,
    }).signers([owner]).rpc();
    console.log("🤖create market transaction signature 🤖", tx);
    // End of Create market //////////////////////////////////////////////////////////////////////////////////////////////////////////////
  });

  it("Deposit liquidity", async () => {
    const market = PublicKey.findProgramAddressSync([Buffer.from(MARKET_SEED), owner.publicKey.toBuffer()], PREDICTION_ID)[0];
    const tx = await program.methods.addLiquidity(new BN(0.05 * 10 ** 9)).accounts({
      user: owner.publicKey,
      creator: owner.publicKey,
      market,
      feeAuthority: owner.publicKey,
      global,
      systemProgram: SystemProgram.programId,
    }).signers([owner]).rpc();
    console.log("🤖Add liquidity transaction signature 🤖", tx);
  });
});
