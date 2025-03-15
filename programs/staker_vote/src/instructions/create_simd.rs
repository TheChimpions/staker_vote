use anchor_lang::prelude::*;
use crate::state::*;
use crate::context::CreateSimd;
use crate::error::ErrorCode;

pub fn handler(ctx: Context<CreateSimd>, name: String, token: Pubkey, yes_account: Pubkey, no_account: Pubkey, abstain_account: Pubkey, link: String, description: String) -> Result<()> {
  // let admins: [Pubkey; 1] = [
  //   Pubkey::from_str("9nKHNaA7RNL7nTk8LwRDWfWJPQ1AgBFy6qKpug2SkAZc").unwrap()
  // ];
  // require!(admins.contains(&*ctx.accounts.user.key), "User not authorized");
  require!(name.len() <= 32, ErrorCode::SimdNameTooLong);

  let simd: &mut Account<'_, Simd> = &mut ctx.accounts.simd;
  simd.name = name;
  simd.token = token;
  simd.yes_account = yes_account;
  simd.no_account = no_account;
  simd.abstain_account = abstain_account;
  simd.link = link;
  simd.description = description;
  Ok(())
}