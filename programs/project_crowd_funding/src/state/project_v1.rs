
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
    pub status: ProjectStatus,
	pub bump: u8,
}

impl ProjectState {
	pub const LEN: usize = 8 // project_id
    + 32
    + 8
    + 8
    + 8
    + 8
    + 1 
    + 32
    + 1 
    + (5 * ContributionTier::LEN); // Up to 5 contribution tiers
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ContributionTier {
    pub tier_id: u64,
    pub amount: u64,
}

impl ContributionTier {
    pub const LEN: usize = 8 + 8; // 16 bytes (tier_id + amount)
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum ProjectStatus {
    Draft,
    Published,
    Successful,
    SoldOut,
    Failed,
    Failing,
}
