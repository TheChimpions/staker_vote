use anchor_lang::prelude::*;
use crate::error::Error;
use crate::context::Vote;

pub fn vote(ctx: Context<Vote>, index: u64, amount: u64, destination: Pubkey) -> Result<()> {
  let vote_status = &mut ctx.accounts.vote_status;
  require!(
    vote_status.voted_amount < amount,
    Error::NoVotesRemaining
  );

  // let node = anchor_lang::solana_program::keccak::hashv(
  //   &index.to_le_bytes(),

  // )

  Ok(())
}