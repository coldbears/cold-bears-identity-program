use anchor_lang::prelude::*;

use crate::{account_states::*, constants::ADMIN_PUBKEY_STR, errors::ErrorCode};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SetIdentityPriceIx {
    pub price: u64,
}

#[derive(Accounts)]
pub struct SetIdentityPriceCtx<'info> {
    #[account(
        mut,
        constraint = admin.key.to_string() == ADMIN_PUBKEY_STR  @ ErrorCode::NotAdmin
    )]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub identity_manager: Box<Account<'info, IdentityManager>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<SetIdentityPriceCtx>, ix: SetIdentityPriceIx) -> Result<()> {
    let identity_manager = &mut ctx.accounts.identity_manager;
    identity_manager.identity_price = ix.price;
    Ok(())
}
