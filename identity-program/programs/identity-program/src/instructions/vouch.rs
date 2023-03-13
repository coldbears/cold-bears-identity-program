use anchor_lang::prelude::*;

use crate::account_states::*;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct VouchCtx<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut, 
        constraint = voucher.owner == authority.key() @ ErrorCode::InvalidIdentityOwner,
        realloc = Identity::space(voucher.rx_vouches.len(), voucher.tx_vouches.len() + 1),
        realloc::payer = authority,
        realloc::zero = false,
    )]
    pub voucher: Box<Account<'info, Identity>>,

    #[account(
        mut,
        constraint = vouchee.owner != voucher.owner @ ErrorCode::VoucheeEqualVoucher,
        realloc = Identity::space(vouchee.rx_vouches.len() + 1, vouchee.tx_vouches.len()),
        realloc::payer = authority,
        realloc::zero = false,
    )]
    pub vouchee: Box<Account<'info, Identity>>,

    #[account(mut)]
    pub identity_manager: Box<Account<'info, IdentityManager>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<VouchCtx>) -> Result<()> {
    let vouchee = &mut ctx.accounts.vouchee;
    let voucher = &mut ctx.accounts.voucher;

    for rx_vouch in &vouchee.rx_vouches {
        // Check if voucher already vouched for vouchee.
        if rx_vouch.identity_id == voucher.id {
            return Err(error!(ErrorCode::AlreadyVouched))
        }
    }

    vouchee.num_of_vouches += 1;
    vouchee.pts_vouches += ctx.accounts.identity_manager.vouch_pts_reward;

    let unix_timestamp = Clock::get().unwrap().unix_timestamp;

    // Register vouchee vouch.
    vouchee.rx_vouches.push(Vouch::new(voucher.id, unix_timestamp));
    // Register vouchers vouch.
    voucher.tx_vouches.push(Vouch::new(vouchee.id, unix_timestamp));

    Ok(())
}
