use anchor_lang::prelude::*;
use crate::state::*;
use crate::context::CreateValidator;
use crate::error::Error;


pub fn handler(ctx: Context<CreateValidator>, name: String, identity_account: Pubkey, vote_account: Pubkey, admin: Pubkey,) -> Result<()> {
  // let admins: [Pubkey; 1] = [
  //   Pubkey::from_str("9nKHNaA7RNL7nTk8LwRDWfWJPQ1AgBFy6qKpug2SkAZc").unwrap()
  // ];
  // require!(admins.contains(&*ctx.accounts.user.key), "User not authorized");
  require!(name.len() <= 32, Error::NameTooLong);
  let stake_vote: &mut Account<'_, StakeVote> = &mut ctx.accounts.stake_vote;
  require!(stake_vote.admins.contains(&*ctx.accounts.user.key), Error::UserNotAuthorized);
  let validator: &mut Account<'_, Validator> = &mut ctx.accounts.validator;
  validator.name = name;
  validator.identity_account = identity_account;
  validator.vote_account = vote_account;
  validator.admin = admin;
  stake_vote.validator_count += 1;
  Ok(())
}