use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct CertProof {
    pub provider: Pubkey,
    pub course: Pubkey,
    pub root: [u8; 32],
    pub total: u64,
    pub claimed: u64,
    pub bump: u8,
}
