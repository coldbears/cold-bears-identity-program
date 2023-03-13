use anchor_lang::prelude::*;

use crate::{account_states::*, constants::ADMIN_PUBKEY_STR, errors::ErrorCode};

#[derive(Accounts)]
pub struct InitIdentityManagerCtx<'info> {
    #[account(
        mut,
        constraint = admin.key.to_string() == ADMIN_PUBKEY_STR @ ErrorCode::NotAdmin
    )]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        seeds = [b"identity-manager", admin.key().as_ref()],
        bump,
        space = IdentityManager::space()
    )]
    pub identity_manager: Box<Account<'info, IdentityManager>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitIdentityManagerCtx>) -> Result<()> {
    let identity_manager = &mut ctx.accounts.identity_manager;
    identity_manager.next_identity_id = 1;
    identity_manager.vouch_pts_reward = 2;
    identity_manager.next_proof_of_online_seed_id = 1;
    identity_manager.is_identity_owner_transfer_enabled = false;
    identity_manager.allow_init_identity = false;
    // 0.2 SOL is default
    identity_manager.identity_price = 200000000;
    
    Ok(())
}
