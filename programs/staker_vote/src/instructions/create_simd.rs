use anchor_lang::prelude::*;
use crate::state::*;
use crate::context::CreateSimd;
use crate::error::Error;

pub fn handler(ctx: Context<CreateSimd>, name: String, token: Pubkey, yes_account: Pubkey, no_account: Pubkey, abstain_account: Pubkey, link: String, description: String) -> Result<()> {
  require!(name.len() <= 32, Error::NameTooLong);
  let staker_vote: &mut Account<'_, StakeVote> = &mut ctx.accounts.stake_vote;
  require!(staker_vote.admins.contains(&*ctx.accounts.user.key), Error::UserNotAuthorized);
  let simd: &mut Account<'_, Simd> = &mut ctx.accounts.simd;
  staker_vote.simd_count += 1;
  simd.name = name;
  simd.token = token;
  simd.yes_account = yes_account;
  simd.no_account = no_account;
  simd.abstain_account = abstain_account;
  simd.link = link;
  simd.description = description;
  Ok(())
}