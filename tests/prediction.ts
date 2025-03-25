import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Prediction } from "../target/types/prediction";
import { PublicKey, SystemProgram, Keypair } from "@solana/web3.js";
import { GLOBAL_SEED, PREDICTION_ID } from "./const";

let global: PublicKey;
let owner: Keypair;
describe("prediction", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  owner = anchor.Wallet.local().payer;

  const program = anchor.workspace.Prediction as Program<Prediction>;
  before(async () => {
    global = PublicKey.findProgramAddressSync([Buffer.from(GLOBAL_SEED)], PREDICTION_ID)[0];
  });
  
  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().accounts({
      global,
      payer: owner.publicKey,
      systemProgram: SystemProgram.programId,
    }).signers([owner]).rpc();
    console.log("Your transaction signature", tx);
  });
});
