use anchor_lang::prelude::*;

use crate::{COURSE_SEED, Course, PROVIDER_SEED, Provider};

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct CreateCourse<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
      has_one = authority,
      seeds = [PROVIDER_SEED, provider.id.to_le_bytes().as_ref()],
      bump = provider.bump,
    )]
    pub provider: Account<'info, Provider>,
    #[account(
      init, 
      payer = authority, 
      space = 8 + Course::INIT_SPACE,
      seeds = [COURSE_SEED, provider.key().as_ref(), id.to_le_bytes().as_ref()],
      bump,
    )]
    pub course: Account<'info, Course>,
    pub system_program: Program<'info, System>,
}


pub fn handler(ctx: Context<CreateCourse>, id: u64, short_name: String) -> Result<()> {
    ctx.accounts.course.set_inner(Course { 
      id, 
      provider: ctx.accounts.provider.key(),
      bump: ctx.bumps.course, 
      short_name 
    });
    Ok(())
}