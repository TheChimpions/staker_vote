use anchor_lang::prelude::*;

use crate::Poll;
use crate::Simd;
use crate::StakeVote;
use crate::Validator;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 64 + 1024)]
    pub stake_vote: Account<'info, StakeVote>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateSimd<'info> {
  #[account(
    init,
    payer = user,
    space = 8 + (4 + 32) + 32 + 32 + 32 + 32 + (4 + 32) + (4 + 32),
    seeds=[b"simd", name.as_bytes()],
    bump
  )]
  pub simd: Account<'info, Simd>,
  #[account(mut)]
  pub user: Signer<'info>,
  #[account(mut)]
  pub stake_vote: Account<'info, StakeVote>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateValidator<'info> {
  #[account(
    init,
    payer = user,
    space = 8 + (4 + 32) + 32 + 32 + 32,
    seeds=[b"validator", name.as_bytes()],
    bump,
  )]
  pub validator: Account<'info, Validator>,
  #[account(mut)]
  pub user: Signer<'info>,
  #[account(mut)]
  pub stake_vote: Account<'info, StakeVote>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddAdmin<'info> {
  #[account(mut)]
  pub stake_vote: Account<'info, StakeVote>,
  #[account(mut)]
  pub user: Signer<'info>,
}

#[derive(Accounts)]

pub struct CreatePoll<'info> {
  #[account(
    init,
    payer = user,
    space = 8 + 8 + 8 + 32 + (4 + 128) + (1 + 0) + 8 + 8 + 8,
    seeds=[b"poll", validator.key().as_ref(), simd.key().as_ref()],
    bump,
  )]
  pub poll: Account<'info, Poll>,
  #[account(mut)]
  pub validator: Account<'info, Validator>,
  #[account(mut)]
  pub simd: Account<'info, Simd>,
  #[account(mut)]
  pub stake_vote: Account<'info, StakeVote>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program<'info, System>,
}