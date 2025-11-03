use anchor_lang::prelude::*;

use crate::{CERT_PROOF_SEED, COURSE_SEED, CertProof, Course, PROVIDER_SEED, Provider};

#[derive(Accounts)]
pub struct CreateCertProof<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
      seeds = [PROVIDER_SEED, provider.id.to_le_bytes().as_ref()],
      bump = provider.bump,
    )]
    pub provider: Account<'info, Provider>,
    #[account(
      seeds = [COURSE_SEED, course.provider.as_ref(), course.id.to_le_bytes().as_ref()],
      bump = course.bump,
      has_one = provider,
    )]
    pub course: Account<'info, Course>,
    #[account(
      init, 
      payer = authority, 
      space = 8 + CertProof::INIT_SPACE,
      seeds = [CERT_PROOF_SEED, provider.key().as_ref(), course.key().as_ref()],
      bump
    )]
    pub cert_proof: Account<'info, CertProof>,
    pub system_program: Program<'info, System>,
}


pub fn handler(ctx: Context<CreateCertProof>, root: [u8; 32], total: u64) -> Result<()> {
    ctx.accounts.cert_proof.set_inner(CertProof { 
      provider: ctx.accounts.provider.key(), 
      course: ctx.accounts.course.key(), 
      root, 
      total, 
      claimed: 0, 
      bump: ctx.bumps.cert_proof 
    });
    Ok(())
}