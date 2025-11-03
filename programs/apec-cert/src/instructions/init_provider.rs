use anchor_lang::prelude::*;

use crate::{PROVIDER_SEED, Provider};

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct InitProvider<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
      init, 
      payer = authority, 
      space = 8 + Provider::INIT_SPACE,
      seeds = [PROVIDER_SEED, id.to_le_bytes().as_ref()],
      bump,
    )]
    pub provider: Account<'info, Provider>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitProvider>, id: u64, short_name: String) -> Result<()> {
    ctx.accounts.provider.set_inner(Provider { 
        id,
        authority: ctx.accounts.authority.key(), 
        bump: ctx.bumps.provider, 
        short_name 
    });
    Ok(())
}
