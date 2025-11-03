use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Course {
    pub id: u64,
    pub provider: Pubkey,
    pub bump: u8,
    #[max_len(20)]
    pub short_name: String,
}
