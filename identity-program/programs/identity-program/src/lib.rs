pub mod account_states;
pub mod constants;
pub mod errors;
pub mod instructions;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("7mzcZ1jT5gKrqYdHuPk3ex8oBHHZpzicYjFDZZhETvL3");

#[program]
pub mod identity_program {
    use super::*;

    pub fn init_identity_manager(ctx: Context<InitIdentityManagerCtx>) -> Result<()> {
        init_identity_manager::handler(ctx)
    }

    pub fn init_identity(ctx: Context<InitIdentityCtx>) -> Result<()> {
        init_identity::handler(ctx)
    }

    pub fn vouch(ctx: Context<VouchCtx>) -> Result<()> {
        vouch::handler(ctx)
    }

    pub fn set_vouch_reward(ctx: Context<SetVouchRewardCtx>, vouch_pts_reward: u64) -> Result<()> {
        set_vouch_reward::handler(ctx, vouch_pts_reward)
    }

    pub fn init_proof_of_online(
        ctx: Context<InitProofOfOnlineCtx>,
        ix: InitProofOfOnlineIx,
    ) -> Result<()> {
        init_proof_of_online::handler(ctx, ix)
    }

    pub fn sign_proof_of_online(ctx: Context<SignProofOfOnlineCtx>) -> Result<()> {
        sign_proof_of_online::handler(ctx)
    }

    pub fn set_identity_owner(
        ctx: Context<UpdateIdentityOwnerCtx>,
        ix: UpdateIdentityOwnerIx,
    ) -> Result<()> {
        update_identity_owner::handler(ctx, ix)
    }

    pub fn set_identity_owner_transferable(
        ctx: Context<SetIdentityOwnerTransferableCtx>,
        ix: SetIdentityOwnerTransferableIx,
    ) -> Result<()> {
        set_identity_owner_transferable::handler(ctx, ix)
    }

    pub fn set_identity_price(
        ctx: Context<SetIdentityPriceCtx>,
        ix: SetIdentityPriceIx,
    ) -> Result<()> {
        set_identity_price::handler(ctx, ix)
    }

    pub fn set_allow_init_identity(
        ctx: Context<SetAllowInitIdentityCtx>,
        ix: SetAllowInitIdentityIx,
    ) -> Result<()> {
        set_allow_init_identity::handler(ctx, ix)
    }
}
