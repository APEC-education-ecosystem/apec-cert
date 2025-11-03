use anchor_lang::prelude::*;

#[error_code]
pub enum ApecErrorCode {
    #[msg("Custom error message")]
    CustomError,
    InvalidProof,
}
