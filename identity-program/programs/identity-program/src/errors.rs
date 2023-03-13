use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid identity owner.")]
    InvalidIdentityOwner,
    #[msg("Voucher can't be equal Vouchee.")]
    VoucheeEqualVoucher,
    #[msg("Voucher have already vouched for vouchee.")]
    AlreadyVouched,
    #[msg("Not admin.")]
    NotAdmin,
    #[msg("Invalid Proof-of-Online. Already ended.")]
    InvalidPoOEnded,
    #[msg("Identity already participated in PoO.")]
    InvalidPoOIdentity,
    #[msg("New owner is already identity owner.")]
    InvalidNewIdentityOwner,
    #[msg("Identity wwner transfer is not enabled in identity manager.")]
    IdentityOwnerTransferNotEnabled,
    #[msg("Invalid treasury address.")]
    InvalidTreasuryAddress,
    #[msg("Initialize of new identity disabled.")]
    InitIdentityDisabled,
}