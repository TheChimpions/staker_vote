use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct VoteRegistry {
  pub base: Pubkey,
  pub admin_auth: Pubkey,
  pub bump: u8,
  pub root: [u8; 32],
  pub mint: Pubkey,
  pub max_votes: u64,
  pub max_num_voters: u64,
  pub total_amount_voted: u64,
  pub num_voters_voted: u64,
  pub validator: Pubkey,
  pub simd: Pubkey,
  pub poll: Pubkey,
}

#[account]
#[derive(Default)]
pub struct VoteStatus {
  pub voter: Pubkey,
  pub voted_at: i64,
  pub voted_amount: u64,
}