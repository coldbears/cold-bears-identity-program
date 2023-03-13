use anchor_lang::prelude::*;

#[account]
pub struct ProofOfOnline {
    // Points each pariticipants will receive.
    pub pts_per_sign: u64,
    // Total number of participants.
    pub number_of_signs: u64,
    // When does the PoO ends in unix.
    pub end_time: i64,
    // A list of identity ids of who have participated in the PoO
    pub participants: Vec<u64>,
}

impl ProofOfOnline {
    pub fn space(len: usize) -> usize {
        8 + (3 * 8) + 4 + ((len + 1) * 8)
    }

    pub fn has_ended(&self, unix_now: i64) -> bool {
        unix_now > self.end_time
    }

    pub fn has_participated(&self, identity_id: &u64) -> bool {
        for id in &self.participants {
            if id == identity_id {
                return true;
            }
        }
        return false;
    }
}
