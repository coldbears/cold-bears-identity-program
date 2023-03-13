use anchor_lang::prelude::*;

use crate::{account_states::*, errors::ErrorCode, constants::ADMIN_PUBKEY_STR};

#[derive(Accounts)]
pub struct SetVouchRewardCtx<'info> {
    #[account(
        mut,
        constraint = admin.key.to_string() == ADMIN_PUBKEY_STR  @ ErrorCode::NotAdmin
    )]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub identity_manager: Box<Account<'info, IdentityManager>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<SetVouchRewardCtx>, vouch_pts_reward: u64) -> Result<()> {
    let identity_manager = &mut ctx.accounts.identity_manager;
    identity_manager.vouch_pts_reward = vouch_pts_reward;
    Ok(())
}
