use anchor_lang::prelude::*;

#[account]
pub struct Validator {
  pub name: String,
  pub identity_account: Pubkey,
  pub vote_account: Pubkey,
  pub admin: Pubkey,
}