use anchor_lang::{prelude::*, system_program};
use anchor_spl::{
    associated_token::spl_associated_token_account::solana_program::keccak,
    token_interface::{
        spl_pod::optional_keys::OptionalNonZeroPubkey,
        spl_token_metadata_interface::state::TokenMetadata,
    },
};

pub fn update_account_minimum_lamports<'info>(
    account: AccountInfo<'info>,
    payer: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
    space: usize,
) -> Result<()> {
    let lamports_required = (Rent::get()?).minimum_balance(space);

    msg!(
        "Update account size with space: {} lamports: {}",
        space as u64,
        lamports_required
    );

    system_program::transfer(
        CpiContext::new(
            system_program,
            system_program::Transfer {
                from: payer,
                to: account,
            },
        ),
        lamports_required,
    )?;
    Ok(())
}

pub fn merkle_verify(proof: Vec<[u8; 32]>, root: [u8; 32], leaf: [u8; 32]) -> bool {
    let mut computed_hash = leaf;
    for proof_element in proof.into_iter() {
        if computed_hash <= proof_element {
            computed_hash = keccak::hashv(&[&computed_hash, &proof_element]).0;
        } else {
            computed_hash = keccak::hashv(&[&proof_element, &computed_hash]).0;
        }
    }
    computed_hash == root
}

pub fn prepare_token_metadata_ext<'info>(
    name: &String,
    symbol: &String,
    uri: &String,
    update_authority: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    payer: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
) -> Result<()> {
    // Implementation for minting an NFT
    let token_metadata = TokenMetadata {
        update_authority: OptionalNonZeroPubkey(update_authority.key()),
        mint: mint.key(),
        name: name.to_string(),
        symbol: symbol.to_string(),
        uri: uri.to_string(),
        ..Default::default()
    };

    let meta_data_space = token_metadata.tlv_size_of().unwrap();

    update_account_minimum_lamports(mint, payer, system_program, meta_data_space)?;
    Ok(())
}
