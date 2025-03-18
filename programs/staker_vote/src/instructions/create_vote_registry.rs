use anchor_lang::prelude::*;

use crate::error::Error;
use crate::context::CreateVoteRegistry;

pub fn handler(
  ctx: Context<CreateVoteRegistry>,
  root: [u8; 32],
  max_votes: u64,
  max_num_voters: u64,
) -> Result<()> {
    let validator: &Account<'_, crate::Validator> = &ctx.accounts.validator;
    require!(validator.admin == *ctx.accounts.user.key, Error::UserNotAuthorized);

    let vote_registry: &mut Account<'_, crate::VoteRegistry> = &mut ctx.accounts.vote_registry;
    vote_registry.validator = ctx.accounts.validator.key();
    vote_registry.simd = ctx.accounts.simd.key();
    vote_registry.base = ctx.accounts.base.key();
    vote_registry.admin_auth = ctx.accounts.admin_auth.key();
    vote_registry.bump = ctx.bumps.vote_registry;
    vote_registry.mint = ctx.accounts.mint.key();
    vote_registry.max_votes = max_votes;
    vote_registry.max_num_voters = max_num_voters;
    vote_registry.total_amount_voted = 0;
    vote_registry.num_voters_voted = 0;
    vote_registry.poll = ctx.accounts.poll.key();
    vote_registry.root = root;

    Ok(())
}