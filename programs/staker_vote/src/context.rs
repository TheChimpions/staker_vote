use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;

use crate::Poll;
use crate::Simd;
use crate::StakeVote;
use crate::Validator;
use crate::VoteRegistry;
use crate::VoteStatus;

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

#[derive(Accounts)]
pub struct CreateVoteRegistry<'info> {
  pub base: Signer<'info>,
  pub admin_auth: Signer<'info>,
  #[account(
    init,
    seeds = [b"VoteRegistry".as_ref(), poll.key().as_ref()],
    bump,
    space = 500,
    payer = user
  )]
  pub vote_registry: Account<'info, VoteRegistry>,

  pub mint: Account<'info, Mint>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program<'info, System>,
  pub poll: Account<'info, Poll>,
  pub validator: Account<'info, Validator>,
  pub simd: Account<'info, Simd>,
}


#[derive(Accounts)]
pub struct Vote<'info> {
  #[account(mut)]
  pub vote_registry: Account<'info, VoteRegistry>,

  #[account(
      init_if_needed,
      seeds = [
        b"VoteStatus".as_ref(),
        vote_registry.key().to_bytes().as_ref(),
        voter.key().to_bytes().as_ref()
      ],
      bump,
      space = 70,
      payer = payer
  )]
  pub vote_status: Account<'info, VoteStatus>,

  #[account(mut)]
  pub from: Account<'info, TokenAccount>,

  #[account(
    init_if_needed,
    payer = payer,
    associated_token::mint = mint,
    associated_token::authority = voter
  )]
  pub to: Account<'info, TokenAccount>,
  pub mint: Account<'info, Mint>,
    /// CHECK: voter is checked in the code to be the same as payer
  pub voter: UncheckedAccount<'info>,
  #[account(mut)]
  pub payer: Signer<'info>,
  pub system_program: Program<'info, System>,
  pub token_program: Program<'info, Token>,
  pub associated_token_program: Program<'info, AssociatedToken>
}