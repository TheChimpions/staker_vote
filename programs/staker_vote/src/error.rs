use anchor_lang::prelude::*;

#[error_code]
pub enum Error {
    #[msg("Name too long")]
    NameTooLong,
    #[msg("Stake vote not initialized")]
    StakeVoteNotInitialized,
    #[msg("User not authorized")]
    UserNotAuthorized,
}
