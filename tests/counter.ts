import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";
import { PublicKey } from "@solana/web3.js";

describe("counter", () => {
  // Configure the client to use the local cluster.

  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Counter as Program<Counter>;

  // const counterAccount = new Keypair();
  const [counterPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("counter")], // This is the seed -- just the string "counter"
    program.programId // If we're interacting with the program, we know its ID
  );

  it("Is initialized!", async () => {
    try {
      // Add your test here.
      const tx = await program.methods
        .initialize()
        .accounts({ counter: counterPDA })
        .rpc();

      const accountData = await program.account.counter.fetch(counterPDA);
      console.log(`Transaction Signature: ${tx}`);
      console.log(`Count: ${accountData.count}`);
    } catch (error) {
      console.error(error);
    }
  });

  it("increment", async () => {
    // invoke the increment method on the programe
    const tx = await program.methods
      .increment()
      .accounts({ counter: counterPDA })
      .rpc();

    const accountData = await program.account.counter.fetch(counterPDA);

    console.log(`Transaction Signature: ${tx}`);
    console.log(`Count: ${accountData.count}`);
  });
});
