use anchor_lang::prelude::*;

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
    seeds=[b"simd", name.as_bytes(), stake_vote.key().as_ref()],
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