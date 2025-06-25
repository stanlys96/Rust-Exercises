import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Voting } from "../target/types/voting";
import { expect } from "chai";

describe("voting", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.voting as Program<Voting>;

  it("Is initialized!", async () => {
    // Add your test here.
    // const tx = await program.methods.initVoting().rpc();
    // console.log("Your transaction signature", tx);
    const [votingPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vote"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );
    console.log(provider.wallet.publicKey.toString());
    console.log(votingPDA.toString());
    console.log(program.programId.toString());
    const votingAccount = await program.account.vote.fetch(votingPDA);
    console.log(votingAccount);
    expect(votingAccount.hasInitiated).eq(true);
  });
});
