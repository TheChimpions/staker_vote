use anchor_lang::prelude::*;

use crate::Simd;

#[derive(Accounts)]
pub struct CreateSimd<'info> {
  #[account(
    init,
    payer = user,
    space = 8 + (4 + 32) + 32 + 32 + 32 + 32 + (4 + 32) + (4 + 32)
  )]
  pub simd: Account<'info, Simd>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program<'info, System>,
}