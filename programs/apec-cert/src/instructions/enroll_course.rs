use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::spl_token_2022::instruction::AuthorityType,
    token_interface::{
        mint_to, set_authority, token_metadata_initialize, Mint, MintTo, SetAuthority,
        TokenAccount, TokenInterface, TokenMetadataInitialize,
    },
};

use crate::{utils::prepare_token_metadata_ext, Course, COURSE_SEED, ENROLLMENT_SEED};

#[derive(Accounts)]
pub struct EnrollCourse<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
      seeds = [COURSE_SEED, course.provider.as_ref(), course.id.to_le_bytes().as_ref()],
      bump = course.bump,
    )]
    pub course: Account<'info, Course>,
    #[account(
      init,
      payer = user,
      mint::decimals = 0,
      mint::token_program = token_program,
      mint::authority = course,
      extensions::close_authority::authority = course,
      extensions::metadata_pointer::authority = course,
      extensions::metadata_pointer::metadata_address = mint,
      seeds = [ENROLLMENT_SEED, course.key().as_ref(), user.key().as_ref()],
      bump,
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
      init,
      payer = user,
      associated_token::token_program = token_program,
      associated_token::mint = mint,
      associated_token::authority = user
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<EnrollCourse>,
    name: String,
    symbol: String,
    uri: String,
) -> Result<()> {
    let course = &ctx.accounts.course;
    let mint = &ctx.accounts.mint;
    prepare_token_metadata_ext(
        &name,
        &symbol,
        &uri,
        course.to_account_info(),
        mint.to_account_info(),
        ctx.accounts.user.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
    )?;

    let seeds = &[
        COURSE_SEED,
        course.provider.as_ref(),
        &course.id.to_le_bytes(),
        &[course.bump],
    ];

    let signer_seeds = &[&seeds[..]];

    token_metadata_initialize(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TokenMetadataInitialize {
                mint: mint.to_account_info(),
                program_id: ctx.accounts.token_program.to_account_info(),
                mint_authority: course.to_account_info(),
                update_authority: course.to_account_info(),
                metadata: ctx.accounts.mint.to_account_info(),
            },
            signer_seeds,
        ),
        name,
        symbol,
        uri,
    )?;

    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: mint.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: course.to_account_info(),
            },
            signer_seeds,
        ),
        1,
    )?;

    // remove mint authority
    set_authority(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            SetAuthority {
                account_or_mint: mint.to_account_info(),
                current_authority: course.to_account_info(),
            },
            signer_seeds,
        ),
        AuthorityType::MintTokens,
        None,
    )?;
    Ok(())
}
