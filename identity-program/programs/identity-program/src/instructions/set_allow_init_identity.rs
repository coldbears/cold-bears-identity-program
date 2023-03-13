use anchor_lang::prelude::*;

use crate::{account_states::*, constants::ADMIN_PUBKEY_STR, errors::ErrorCode};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SetAllowInitIdentityIx {
    pub allow_init_identity: bool,
}

#[derive(Accounts)]
pub struct SetAllowInitIdentityCtx<'info> {
    #[account(
        mut,
        constraint = admin.key.to_string().eq(ADMIN_PUBKEY_STR) @ ErrorCode::NotAdmin
    )]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub identity_manager: Box<Account<'info, IdentityManager>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<SetAllowInitIdentityCtx>, ix: SetAllowInitIdentityIx) -> Result<()> {
    let identity_manager = &mut ctx.accounts.identity_manager;
    identity_manager.allow_init_identity = ix.allow_init_identity;

    Ok(())
}
