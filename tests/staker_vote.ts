import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { StakerVote } from "../target/types/staker_vote";
import { Connection, Keypair } from "@solana/web3.js";
import { createMint, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { equal } from "assert";

describe("staker_vote", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.stakerVote as Program<StakerVote>;
  const programPair = Keypair.generate();
  const deployer = Keypair.generate();
  const staker1 = Keypair.generate();
  const staker2 = Keypair.generate();
  const yesAccount = Keypair.generate();
  const noAccount = Keypair.generate();
  const abstainAccount = Keypair.generate();
  const connection = new Connection("http://localhost:8899", "confirmed");

  async function addFunds(
    user: anchor.web3.Keypair,
    amount: number,
    provider: anchor.Provider
  ) {
    const latestBlockhash = await provider.connection.getLatestBlockhash();
    const tx = await provider.connection.requestAirdrop(user.publicKey, amount);
    await provider.connection.confirmTransaction({
      blockhash: latestBlockhash.blockhash,
      lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
      signature: tx,
    });
  }

  async function createTestToken() {
    const mint = await createMint(
      connection,
      deployer,
      deployer.publicKey,
      null,
      0,
      undefined,
      {},
      TOKEN_PROGRAM_ID
    )
    return mint
  }

  it("creates simd for voting", async () => {
    await addFunds(deployer, 1000000000, anchor.getProvider());
    const mint = await createTestToken()
    await program.methods.createSimd(
      "228", mint,
      yesAccount.publicKey,
      noAccount.publicKey,
      abstainAccount.publicKey,
      "http://chimpions.co",
      "nothingburger"
    ).accounts({simd: programPair.publicKey})
    .signers([programPair])
    .rpc()

    const simd = await program.account.simd.fetch(programPair.publicKey)
    equal(simd.yesAccount.toBase58(), yesAccount.publicKey.toBase58())
    equal(simd.noAccount.toBase58(), noAccount.publicKey.toBase58())
    equal(simd.abstainAccount.toBase58(), abstainAccount.publicKey.toBase58())
    equal(simd.token.toBase58(), mint.toBase58())
    equal(simd.link, "http://chimpions.co")
    equal(simd.description, "nothingburger")
    equal(simd.name, "228")
  });
});
