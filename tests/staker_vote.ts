import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { StakerVote } from "../target/types/staker_vote";
import { Connection, Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { createMint, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import assert, { equal, notEqual } from "assert";
import { use } from "chai";
import { userInfo } from "os";

describe("staker_vote", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.stakerVote as Program<StakerVote>;
  const programPair = Keypair.generate();
  const deployer = Keypair.generate();
  const secondAdmin = Keypair.generate();
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

  async function findSimdAddress(name: string) {
    let [simd_address, _bump] = await PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("simd"),
        anchor.utils.bytes.utf8.encode(name),
      ],
      program.programId
    )
    return simd_address
  };

  async function findSimd(name: string) {
    let simd_address = await findSimdAddress(name)
    let simd = await program.account.simd.fetch(simd_address)
    return simd
  };

  async function findValidatorAddress(name: string) {
    let [validator_address, _bump] = await PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("validator"),
        anchor.utils.bytes.utf8.encode(name),
      ],
      program.programId
    )
    return validator_address
  };

  async function findValidator(name: string) {
    let validator_address = await findValidatorAddress(name)
    let validator = await program.account.validator.fetch(validator_address)
    return validator
  };

  async function findPollAddress(validator_address: PublicKey, simd_address: PublicKey) {
    let [poll_address, _bump] = await PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("poll"),
        validator_address.toBuffer(),
        simd_address.toBuffer(),
      ],
      program.programId
    )
    return poll_address
  };

  async function findPoll(validator_address: PublicKey, simd_address: PublicKey) {
    let poll_address = await findPollAddress(validator_address, simd_address)
    let poll = await program.account.poll.fetch(poll_address)
    return poll
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

  it ("adds a second admin", async () => {
    await program.methods.addAdmin(secondAdmin.publicKey).accounts({
      stakeVote: programPair.publicKey,
      user: deployer.publicKey,
    }).signers([deployer]).rpc();
    const account = await program.account.stakeVote.fetch(programPair.publicKey);
    equal(account.admins.length, 2);
    equal(account.admins[1].toBase58(), secondAdmin.publicKey.toBase58());
  });

  it ("unauthorized user cannot add an admin", async () => {
    try {
      await program.methods.addAdmin(staker1.publicKey).accounts({
        stakeVote: programPair.publicKey,
        user: staker1.publicKey,
      }).signers([staker1]).rpc();
      assert.ok(false); // Should not reach this line
    } catch (err) {
      assert(err.message.includes("User not authorized"));
      return
    }
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
    let simd = await findSimd("228");
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

    let validator = await findValidator("Chimpions")
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

  it ("creates a poll for simd", async () => {
    let simd_address = await findSimdAddress("228")
    let validator_address= await findValidatorAddress("Chimpions")

    await program.methods.createPoll(
      abstainAccount.publicKey,
      "No comment",
    )
    .accounts({stakeVote: programPair.publicKey, user: deployer.publicKey, simd: simd_address, validator: validator_address})
    .signers([deployer])
    .rpc()

    let poll = await findPoll(validator_address, simd_address);

    notEqual(poll.status.setup, null)
    equal(poll.default, abstainAccount.publicKey.toBase58())
    equal(poll.yesVotes, 0)
    equal(poll.noVotes, 0)
    equal(poll.abstainVotes, 0)
    equal(poll.validatorsPosition, "No comment")
    equal(poll.simd.toBase58(), simd_address.toBase58())
    equal(poll.validator.toBase58(), validator_address.toBase58())
  });

  // it ("creates merkle tree for poll", async () => {
  //   const mint = await createTestToken()
  //   let [simd_address, _bump] = await PublicKey.findProgramAddressSync(
  //     [
  //       anchor.utils.bytes.utf8.encode("simd"),
  //       anchor.utils.bytes.utf8.encode("228"),
  //     ],
  //     program.programId
  //   )
  //   let [validator_address, _bump2] = await PublicKey.findProgramAddressSync(
  //     [
  //       anchor.utils.bytes.utf8.encode("validator"),
  //       anchor.utils.bytes.utf8.encode("Chimpions"),
  //     ],
  //     program.programId
  //   )
  //   let [poll_address, _bump3] = await PublicKey.findProgramAddressSync(
  //     [
  //       anchor.utils.bytes.utf8.encode("poll"),
  //       validator_address.toBuffer(),
  //       simd_address.toBuffer(),
  //     ],
  //     program.programId
  //   )
  // });
});
