use anchor_lang::prelude::*;

use crate::{account_states::*, constants::ADMIN_PUBKEY_STR, errors::ErrorCode};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SetIdentityOwnerTransferableIx {
    pub is_transferable: bool,
}

#[derive(Accounts)]
pub struct SetIdentityOwnerTransferableCtx<'info> {
    #[account(
        mut,
        constraint = admin.key.to_string() == ADMIN_PUBKEY_STR  @ ErrorCode::NotAdmin
    )]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub identity_manager: Box<Account<'info, IdentityManager>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<SetIdentityOwnerTransferableCtx>,
    ix: SetIdentityOwnerTransferableIx,
) -> Result<()> {
    let identity_manager = &mut ctx.accounts.identity_manager;
    identity_manager.is_identity_owner_transfer_enabled = ix.is_transferable;

    Ok(())
}
