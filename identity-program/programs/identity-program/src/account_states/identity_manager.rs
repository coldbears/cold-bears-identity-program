use anchor_lang::prelude::*;

#[account]
pub struct IdentityManager {
    pub next_identity_id: u64,
    pub vouch_pts_reward: u64,
    pub next_proof_of_online_seed_id: u64,
    pub is_identity_owner_transfer_enabled: bool,
    pub identity_price: u64,
    pub allow_init_identity: bool
}


impl IdentityManager {
    pub fn space() -> usize {
        8 + 8 + 8 + 8 + 1 + 8 + 1
    }
}