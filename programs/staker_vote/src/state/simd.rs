use anchor_lang::prelude::*;

#[account]
pub struct Simd {
  pub name: String,
  pub token: Pubkey,
  pub yes_account: Pubkey,
  pub no_account: Pubkey,
  pub abstain_account: Pubkey,
  pub link: String,
  pub description: String,
}
