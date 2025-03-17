use anchor_lang::prelude::*;

use crate::{context::AddAdmin, StakeVote};
use crate::error::Error;

pub fn handler(ctx: Context<AddAdmin>, new_admin: Pubkey) -> Result<()> {
  let stake_vote: &mut Account<'_, StakeVote> = &mut ctx.accounts.stake_vote;
  require!(stake_vote.admins.contains(&*ctx.accounts.user.key), Error::UserNotAuthorized);
  stake_vote.admins.push(new_admin);
  Ok(())
}