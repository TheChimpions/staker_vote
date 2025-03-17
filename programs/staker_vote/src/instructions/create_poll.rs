use anchor_lang::prelude::*;
use crate::state::*;
use crate::context::CreatePoll;

pub fn handler(
  ctx: Context<CreatePoll>,
  default: Pubkey,
  validators_position: String,
) -> Result<()> {
  let poll: &mut Account<'_, Poll> = &mut ctx.accounts.poll;
  let validator: &mut Account<'_, Validator> = &mut ctx.accounts.validator;
  let simd: &mut Account<'_, Simd> = &mut ctx.accounts.simd;

  poll.default = default;
  poll.validators_position = validators_position;
  poll.status = Stage::Setup;
  poll.yes_votes = 0;
  poll.no_votes = 0;
  poll.abstain_votes = 0;
  poll.simd = simd.key();
  poll.validator = validator.key();
  Ok(())
}