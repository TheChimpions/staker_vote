pub mod constants;
pub mod error;
pub mod context;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

use crate::context::*;
declare_id!("HePAkr7Tyqtb6ApARurxhAnMzCMJLKn8nUSFo9jbFhrF");

#[program]
pub mod staker_vote {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn add_admin(ctx: Context<AddAdmin>, new_admin: Pubkey) -> Result<()> {
        add_admin::handler(ctx, new_admin)
    }

    pub fn create_simd(ctx: Context<CreateSimd>, name: String, token: Pubkey, yes_account: Pubkey, no_account: Pubkey, abstain_account: Pubkey, link: String, description: String) -> Result<()> {
        create_simd::handler(ctx, name, token, yes_account, no_account, abstain_account, link, description)
    }

    pub fn create_validator(ctx: Context<CreateValidator>, name: String, identity_account: Pubkey, vote_account: Pubkey, admin: Pubkey,) -> Result<()> {
        create_validator::handler(ctx, name, identity_account, vote_account, admin)
    }
}
