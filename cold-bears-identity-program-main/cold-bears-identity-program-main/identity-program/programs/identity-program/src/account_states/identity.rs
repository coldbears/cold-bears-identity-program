use anchor_lang::prelude::*;

#[account]
pub struct Identity {
    pub id: u64,
    // Owner of the identity. Can be changed but not by default!
    pub owner: Pubkey,
    // Number of vouches.
    pub num_of_vouches: u64,
    // The reward in pts from a vouch is adjustable.
    pub pts_vouches: u64,
    // Number of participation in the PoO.
    pub num_of_proof_of_online: u64,
    // Each PoO may reward the user differently, so we need to track the pts reward.
    pub pts_proof_of_online: u64,
    // Received vouches.
    pub rx_vouches: Vec<Vouch>,
    // Sent vouches.
    pub tx_vouches: Vec<Vouch>,
    // Unix time the identity was created.
    pub created_at: i64,
    // Slot the identity was created.
    pub slot: u64,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Vouch {
    // Id of identity that made the vouch.
    pub identity_id: u64,
    // Time the vouched happened.
    pub vouched_at: i64,
}

impl Identity {
    pub fn space(rx_vouches_len: usize, tx_vouches_len: usize) -> usize {
        8 + 8 + 32 + 8 + 8 + 8 + 8 + 8 + 8 + Vouch::space(rx_vouches_len) + Vouch::space(tx_vouches_len)
    }

    pub fn clone_data(&mut self, old: &Identity, new_owner: Pubkey) {
        self.id = old.id;
        self.owner = new_owner;
        self.num_of_vouches = old.num_of_vouches;
        self.pts_vouches = old.pts_vouches;
        self.num_of_proof_of_online = old.num_of_proof_of_online;
        self.pts_proof_of_online = old.pts_proof_of_online;
        self.rx_vouches = old.rx_vouches.clone();
        self.tx_vouches = old.tx_vouches.clone();
        self.created_at = old.created_at;
    }
}

impl Vouch {
    pub fn space(len: usize) -> usize {
        ((8 + 8) * (1 + len)) + 4
    }
    pub fn new(identity_id: u64, vouched_at: i64) -> Self {
        Self {
            identity_id,
            vouched_at,
        }
    }
}
