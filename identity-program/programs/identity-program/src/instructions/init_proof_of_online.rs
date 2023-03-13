use anchor_lang::prelude::*;

use crate::errors::ErrorCode;
use crate::{account_states::*, constants::ADMIN_PUBKEY_STR};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitProofOfOnlineIx {
    pub pts_per_sign: u64,
    pub end_time: i64,
}

#[derive(Accounts)]
pub struct InitProofOfOnlineCtx<'info> {
    #[account(
        mut,
        constraint = admin.key.to_string() == ADMIN_PUBKEY_STR @ ErrorCode::NotAdmin
    )]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        seeds = [
            b"proof-of-online",
            admin.key().as_ref(),
            identity_manager.next_proof_of_online_seed_id.to_le_bytes().as_ref()
        ],
        bump,
        space = ProofOfOnline::space(0)
    )]
    pub proof_of_online: Box<Account<'info, ProofOfOnline>>,

    #[account(mut)]
    pub identity_manager: Box<Account<'info, IdentityManager>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitProofOfOnlineCtx>, ix: InitProofOfOnlineIx) -> Result<()> {
    let identity_manager = &mut ctx.accounts.identity_manager;
    let proof_of_online = &mut ctx.accounts.proof_of_online;

    identity_manager.next_proof_of_online_seed_id += 1;

    proof_of_online.pts_per_sign = ix.pts_per_sign;
    proof_of_online.end_time = ix.end_time;

    Ok(())
}
