import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { StakerVote } from "../target/types/staker_vote";
import { Connection, Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { createMint, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import assert, { equal } from "assert";
import { use } from "chai";
import { userInfo } from "os";

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
  const validator_identity = Keypair.generate();
  const validator_vote = Keypair.generate();
  const validator_admin = Keypair.generate();
  const validator_account = Keypair.generate();
  const simd228 = Keypair.generate();
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

  it ("initializes the program", async () => {
    await addFunds(deployer, 1000000000, anchor.getProvider());
    const tx = await program.methods.initialize().accounts({
      stakeVote: programPair.publicKey,
      user: deployer.publicKey,
    }).signers([deployer, programPair]).rpc();
    const account = await program.account.stakeVote.fetch(programPair.publicKey);
    equal(account.simdCount, 0);
    equal(account.validatorCount, 0);
    equal(account.voteCount, 0);
    equal(account.admins[0].toBase58(), deployer.publicKey.toBase58());
  });

  it("creates simd for voting", async () => {
    const mint = await createTestToken()
    await program.methods.createSimd(
      "228", mint,
      yesAccount.publicKey,
      noAccount.publicKey,
      abstainAccount.publicKey,
      "http://chimpions.co",
      "nothingburger"
    ).accounts({stakeVote: programPair.publicKey, user: deployer.publicKey})
    .signers([deployer])
    .rpc()

    const account = await program.account.stakeVote.fetch(programPair.publicKey);
    let [simd_address, _bump] = await PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("simd"),
        anchor.utils.bytes.utf8.encode("228"),
      ],
      program.programId
    )

    let simd = await program.account.simd.fetch(simd_address)
    equal(simd.yesAccount.toBase58(), yesAccount.publicKey.toBase58())
    equal(simd.noAccount.toBase58(), noAccount.publicKey.toBase58())
    equal(simd.abstainAccount.toBase58(), abstainAccount.publicKey.toBase58())
    equal(simd.token.toBase58(), mint.toBase58())
    equal(simd.link, "http://chimpions.co")
    equal(simd.description, "nothingburger")
    equal(simd.name, "228")
    equal(account.simdCount, 1);
  });

  it ("doesn't allow non admin to create simd", async () => {
    addFunds(staker1, 1000000000, anchor.getProvider());
    const mint = await createTestToken()
    try {
      const tx = await program.methods.createSimd(
        "sketchy_proposal", mint,
        yesAccount.publicKey,
        noAccount.publicKey,
        abstainAccount.publicKey,
        "http://chimpions.co",
        "nothingburger"
      ).accounts({stakeVote: programPair.publicKey, user: staker1.publicKey})
      .signers([staker1])
      .rpc()
      assert.ok(false); // Should not reach this line
    } catch (err) {
      assert(err.message.includes("User not authorized"));
      return
    }
  });

  it("creates a validator", async () => {
    await program.methods.createValidator(
      "Chimpions",
      validator_identity.publicKey,
      validator_vote.publicKey,
      validator_admin.publicKey
    )
    .accounts({user: deployer.publicKey, stakeVote: programPair.publicKey,})
    .signers([deployer])
    .rpc()

    let [validator_address, _bump] = await PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("validator"),
        anchor.utils.bytes.utf8.encode("Chimpions"),
      ],
      program.programId
    )

    let validator = await program.account.validator.fetch(validator_address)
    equal(validator.identityAccount.toBase58(), validator_identity.publicKey.toBase58())
    equal(validator.voteAccount.toBase58(), validator_vote.publicKey.toBase58())
    equal(validator.admin.toBase58(), validator_admin.publicKey.toBase58())
    equal(validator.name, "Chimpions")
    const account = await program.account.stakeVote.fetch(programPair.publicKey);
    equal(account.validatorCount, 1);
  });

  it ("doesn't allow non admin to create validator", async () => {
    try {
      await program.methods.createValidator(
        "Fake Chimpions",
        validator_identity.publicKey,
        validator_vote.publicKey,
        validator_admin.publicKey
      )
      .accounts({user: staker1.publicKey, stakeVote: programPair.publicKey,})
      .signers([staker1])
      .rpc()
      assert.ok(false); // Should not reach this line
    } catch (err) {
      assert(err.message.includes("User not authorized"));
      return
    }
  });
});
