use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Provider {
    pub id: u64,
    pub authority: Pubkey,
    pub bump: u8,
    #[max_len(10)]
    pub short_name: String,
}
