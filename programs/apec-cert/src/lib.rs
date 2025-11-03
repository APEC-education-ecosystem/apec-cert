pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;
use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("CAxe8JydEaRrtF3DVdPATw9XwAgYZUnCJ4wr5ZbvUFMp");

#[program]
pub mod apec_cert {
    use super::*;

    pub fn init_provider(ctx: Context<InitProvider>, id: u64, short_name: String) -> Result<()> {
        instructions::init_provider::handler(ctx, id, short_name)
    }

    pub fn create_course(ctx: Context<CreateCourse>, id: u64, short_name: String) -> Result<()> {
        instructions::create_course::handler(ctx, id, short_name)
    }

    pub fn enroll_course(
        ctx: Context<EnrollCourse>,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        instructions::enroll_course::handler(ctx, name, symbol, uri)
    }

    pub fn create_cert_proof(
        ctx: Context<CreateCertProof>,
        root: [u8; 32],
        total: u64,
    ) -> Result<()> {
        instructions::create_cert_proof::handler(ctx, root, total)
    }

    pub fn claim_cert(
        ctx: Context<ClaimCert>,
        proof: Vec<[u8; 32]>,
        name: String,
        uri: String,
    ) -> Result<()> {
        instructions::claim_cert::handler(ctx, proof, name, uri)
    }
}
