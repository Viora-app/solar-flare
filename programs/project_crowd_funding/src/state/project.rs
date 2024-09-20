use anchor_lang::prelude::*;

#[account]
pub struct ProjectState {
    pub project_id: u64,
    pub owner: Pubkey,
    pub soft_cap: u64,
    pub hard_cap: u64,
    pub deadline: i64,
    pub current_funding: u64,
    pub contribution_tiers: Vec<ContributionTier>,
    pub contributions: Vec<Contribution>,
    pub status: ProjectStatus,
    pub muzikie_address: Pubkey, // Muzikie's wallet address
    pub wallet_address: Pubkey,  // Muzikie's wallet address
}

impl ProjectState {
    pub const LEN: usize = 8
        + 32
        + 32
        + 32
        + 8
        + 8
        + 8
        + 8
        + 1
        + (10 * ContributionTier::LEN)
        + (10 * Contribution::LEN);
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ContributionTier {
    pub tier_id: u64,
    pub amount: u64,
}

impl ContributionTier {
    pub const LEN: usize = 8 + 8;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Contribution {
    pub sender_address: Pubkey,
    pub contribution_tier_id: u64,
}

impl Contribution {
    pub const LEN: usize = 32 + 8;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ProjectStatus {
    Draft,
    Live,
    Successful,
    Failed,
    Final,
}
