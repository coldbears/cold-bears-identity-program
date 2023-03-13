use anchor_lang::prelude::*;

use crate::account_states::*;
use crate::errors::ErrorCode;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateIdentityOwnerIx {
    pub new_owner: Pubkey,
}

#[derive(Accounts)]
#[instruction(new_owner: Pubkey)]
pub struct UpdateIdentityOwnerCtx<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut, 
        constraint = identity.owner == authority.key() @ ErrorCode::InvalidIdentityOwner,
        constraint = identity.owner != new_owner @ ErrorCode::InvalidNewIdentityOwner,
        close = authority
    )]
    pub identity: Box<Account<'info, Identity>>,

    #[account(
        init,
        payer = authority,
        seeds = [b"identity", new_owner.as_ref()],
        bump,
        space = Identity::space(identity.rx_vouches.len(),identity.tx_vouches.len())
    )]
    pub new_identity: Box<Account<'info, Identity>>,

    #[account(mut)]
    pub identity_manager: Box<Account<'info, IdentityManager>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<UpdateIdentityOwnerCtx>, ix: UpdateIdentityOwnerIx) -> Result<()> {
    if !ctx.accounts.identity_manager.is_identity_owner_transfer_enabled {
        return Err(error!(ErrorCode::IdentityOwnerTransferNotEnabled))
    }
    let identity = &mut ctx.accounts.identity;
    let new_identity = &mut ctx.accounts.new_identity;

    new_identity.clone_data(identity, ix.new_owner);
    Ok(())
}
