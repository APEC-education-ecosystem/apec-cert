use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::{spl_associated_token_account::solana_program::keccak, AssociatedToken},
    token_2022::{spl_token_2022::instruction::AuthorityType, ID as TOKEN_2022_PROGRAM_ID},
    token_interface::{
        mint_to, set_authority, token_metadata_initialize, Mint, MintTo, SetAuthority,
        TokenAccount, TokenInterface, TokenMetadataInitialize,
    },
};

use crate::{
    error::ApecErrorCode,
    utils::{merkle_verify, prepare_token_metadata_ext},
    CertProof, Course, Provider, CERTIFICATE_SEED, PROVIDER_SEED,
};

#[derive(Accounts)]
pub struct ClaimCert<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub claimer: SystemAccount<'info>,
    pub provider: Account<'info, Provider>,
    #[account(
      has_one = provider,
    )]
    pub course: Account<'info, Course>,
    #[account(
      has_one = provider,
    )]
    pub cert_proof: Account<'info, CertProof>,
    #[account(
      init,
      payer = payer,
      mint::decimals = 0,
      mint::token_program = token_program,
      mint::authority = provider,
      extensions::close_authority::authority = provider,
      extensions::metadata_pointer::authority = provider,
      extensions::metadata_pointer::metadata_address = cert_mint,
      seeds = [CERTIFICATE_SEED, cert_proof.provider.as_ref(), cert_proof.course.as_ref(), claimer.key().as_ref()],
      bump,
    )]
    pub cert_mint: InterfaceAccount<'info, Mint>,
    #[account(
      init,
      payer = payer,
      associated_token::token_program = token_program,
      associated_token::mint = cert_mint,
      associated_token::authority = claimer
    )]
    pub claimer_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
      address = TOKEN_2022_PROGRAM_ID,
    )]
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<ClaimCert>,
    proof: Vec<[u8; 32]>,
    name: String,
    uri: String,
) -> Result<()> {
    let claimer = &mut ctx.accounts.claimer;
    let cert_proof = &mut ctx.accounts.cert_proof;
    let provider = &ctx.accounts.provider;
    let mint = &ctx.accounts.cert_mint;

    let leaf = keccak::hashv(&[claimer.key().as_array()]).0;

    require!(
        merkle_verify(proof, cert_proof.root, leaf),
        ApecErrorCode::InvalidProof,
    );

    let symbol = "APECERT".to_string();

    prepare_token_metadata_ext(
        &name,
        &symbol,
        &uri,
        ctx.accounts.provider.to_account_info(),
        ctx.accounts.cert_mint.to_account_info(),
        ctx.accounts.payer.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
    )?;

    let seeds = &[PROVIDER_SEED, &provider.id.to_le_bytes(), &[provider.bump]];

    let signer_seeds = &[&seeds[..]];

    token_metadata_initialize(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TokenMetadataInitialize {
                mint: mint.to_account_info(),
                program_id: ctx.accounts.token_program.to_account_info(),
                mint_authority: provider.to_account_info(),
                update_authority: provider.to_account_info(),
                metadata: mint.to_account_info(),
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
                to: ctx.accounts.claimer_token_account.to_account_info(),
                authority: provider.to_account_info(),
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
                current_authority: provider.to_account_info(),
            },
            signer_seeds,
        ),
        AuthorityType::MintTokens,
        None,
    )?;

    Ok(())
}
