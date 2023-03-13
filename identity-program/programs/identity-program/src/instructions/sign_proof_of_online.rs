use anchor_lang::prelude::*;

use crate::{account_states::*, errors::ErrorCode};

#[derive(Accounts)]
pub struct SignProofOfOnlineCtx<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        realloc = ProofOfOnline::space((proof_of_online.number_of_signs + 1) as usize),
        realloc::payer = authority,
        realloc::zero = false,
    )]
    pub proof_of_online: Box<Account<'info, ProofOfOnline>>,

    #[account(
        mut,
        constraint = identity.owner == authority.key()
    )]
    pub identity: Box<Account<'info, Identity>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<SignProofOfOnlineCtx>) -> Result<()> {
    let proof_of_online = &mut ctx.accounts.proof_of_online;
    let identity = &mut ctx.accounts.identity;

    if proof_of_online.has_ended(Clock::get().unwrap().unix_timestamp) {
        return Err(error!(ErrorCode::InvalidPoOEnded));
    }

    if proof_of_online.has_participated(&identity.id) {
        return Err(error!(ErrorCode::InvalidPoOIdentity));
    }

    proof_of_online.participants.push(identity.id);
    proof_of_online.number_of_signs += 1;

    identity.num_of_proof_of_online += 1;
    identity.pts_proof_of_online += proof_of_online.pts_per_sign;

    Ok(())
}
