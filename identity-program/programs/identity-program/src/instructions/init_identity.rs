use crate::errors::ErrorCode;
use crate::{account_states::*, constants::{TREASURY_PUBKEY_STR, ADMIN_PUBKEY_STR}};
use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, system_instruction},
};

#[derive(Accounts)]
pub struct InitIdentityCtx<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        seeds = [b"identity", identity_owner.key.as_ref()],
        bump,
        space = Identity::space(0,0)
    )]
    pub identity: Box<Account<'info, Identity>>,

    pub identity_owner: SystemAccount<'info>,

    #[account(
        mut,
        constraint = treasury.key().to_string().eq(TREASURY_PUBKEY_STR) @ ErrorCode::InvalidTreasuryAddress,
    )]
    pub treasury: SystemAccount<'info>,

    #[account(mut)]
    pub identity_manager: Box<Account<'info, IdentityManager>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitIdentityCtx>) -> Result<()> {
    let identity = &mut ctx.accounts.identity;
    let identity_manager = &mut ctx.accounts.identity_manager;

    if !identity_manager.allow_init_identity {
        if !ctx.accounts.authority.key().to_string().eq(ADMIN_PUBKEY_STR) {
            return Err(error!(ErrorCode::InitIdentityDisabled));
        }
    }

    invoke(
        &system_instruction::transfer(
            ctx.accounts.authority.key,
            ctx.accounts.treasury.key,
            identity_manager.identity_price,
        ),
        &[
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.treasury.to_account_info(),
        ],
    )?;

    identity.id = identity_manager.next_identity_id;
    identity.owner = *ctx.accounts.identity_owner.key;
    identity.num_of_vouches = 0;
    identity.pts_vouches = 0;
    identity.num_of_proof_of_online = 0;
    identity.pts_proof_of_online = 0;
    identity.created_at = Clock::get().unwrap().unix_timestamp;
    identity.slot = Clock::get().unwrap().slot;

    identity_manager.next_identity_id += 1;

    Ok(())
}
