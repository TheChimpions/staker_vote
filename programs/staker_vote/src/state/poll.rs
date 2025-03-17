use anchor_lang::prelude::*;

#[account]
pub struct Poll {
  pub default: Pubkey,
  pub validators_position: String,
  pub status: Stage,
  pub yes_votes: u64,
  pub no_votes: u64,
  pub abstain_votes: u64,
  pub simd: Pubkey,
  pub validator: Pubkey,
}

#[derive(AnchorDeserialize, AnchorSerialize, PartialEq, Eq, Clone)]
pub enum Stage {
  Setup,
  Voting,
  Closed
}