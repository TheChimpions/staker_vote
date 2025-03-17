use anchor_lang::prelude::*;

#[account]
pub struct StakeVote {
  pub simd_count: u64,
  pub validator_count: u64,
  pub vote_count: u64,
  pub admins: Vec<Pubkey>,
}