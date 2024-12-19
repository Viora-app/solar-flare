
use anchor_lang::prelude::*;


// Data Account For Project (Campaign)
#[account]
#[derive(InitSpace)]
pub struct Project {
    pub project_id: u64,
    //pub owner: Pubkey,
    pub soft_cap: u64,
    pub hard_cap: u64,
    pub deadline: i64,
    pub current_funding: u64,
    #[max_len(5)] // Limit Vec to a maximum of 5 tiers
    pub contribution_tiers: Vec<ContributionTier>,
    pub status: ProjectStatus,
	pub bump: u8,
}



#[derive(AnchorSerialize, InitSpace, AnchorDeserialize, Clone, Debug)]
pub struct ContributionTier {
    pub tier_id: u64,
    pub amount: u64,
}

#[derive(AnchorSerialize, InitSpace, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum ProjectStatus {
    Draft, // init
    Published, // after adding tiers and publishing 
    Successful, // when soft_cap is reached
    SoldOut, // when hard_cap is reached
    Failing,  // when deadling reached but still does not satisfy soft_cap (and absolutely hard_cap)
    Failed,  // when all of the contributers have been refunded
}
