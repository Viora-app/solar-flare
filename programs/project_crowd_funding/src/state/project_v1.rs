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
    pub escrow: Pubkey, // project's account witch contains the funds
	pub bump: u8,
}

impl ProjectState {
	pub const LEN: usize = 8 // project_id
    + 32 // owner
    + 8 // soft_cap
    + 8 // hard_cap
    + 8 // deadline
    + 8 // current_funding
    + 1 // status (enum is 1 byte)
    + 32 // muzikie_address
    + 1 // bump
    + (5 * ContributionTier::LEN); // Up to 5 contribution tiers
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ContributionTier {
    pub tier_id: u64,   // Unique ID for the tier
    pub amount: u64,    // Amount of SOL for this tier
}

impl ContributionTier {
    pub const LEN: usize = 8 + 8; // 16 bytes (tier_id + amount)
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)] // Add PartialEq here
pub enum ProjectStatus {
    Draft,
    Published,
    Successful,
    SoldOut,
    Failed,
    Reimbursing,
    Failing,
}
