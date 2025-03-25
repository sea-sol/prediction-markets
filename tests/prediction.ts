import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Prediction } from "../target/types/prediction";
import { PublicKey, SystemProgram, Keypair, Transaction, Connection } from "@solana/web3.js";
import { GLOBAL_SEED, PREDICTION_ID, SOL_USDC_FEED } from "./const";

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
      console.log("OracleResUpdated 👻", Number(event.oracleRes));
    });

    const GlobalEvent = await program.addEventListener("GlobalInitialized", (event, slot, signature) => {
      console.log("GlobalInitilized  👻", Number(event));
    });

    global = PublicKey.findProgramAddressSync([Buffer.from(GLOBAL_SEED)], PREDICTION_ID)[0];
  });
  
  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize({
      feeAuthority: owner.publicKey,
      feePercentage: 100,
    }).accounts({
      global,
      payer: owner.publicKey,
      systemProgram: SystemProgram.programId,
    }).signers([owner]).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Get oracle res", async () => {
    const createTx = new Transaction()

    const tx = await program.methods.getRes().accounts({
      user: owner.publicKey,
      feedAggregator: new PublicKey(SOL_USDC_FEED),
      feed: new PublicKey("5mXfTYitRFsWPhdJfp2fc8N6hK8cw6NB5jAYpronQasj"),
      systemProgram: SystemProgram.programId,
    }).signers([owner]).rpc();

    console.log("Your transaction signature", tx);
  });
});
