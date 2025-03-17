use anchor_lang::prelude::*;
use crate::context::Initialize;

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let stake_vote = &mut ctx.accounts.stake_vote;

    stake_vote.simd_count = 0;
    stake_vote.validator_count = 0;
    stake_vote.vote_count = 0;
    stake_vote.admins = vec![*ctx.accounts.user.key];
    Ok(())
}
