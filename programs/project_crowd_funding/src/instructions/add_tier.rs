use anchor_lang::prelude::*;
use crate::state::project::ProjectState; // Correctly import ProjectState from the project module
use crate::state::project::ContributionTier; // Also import ContributionTier for adding tiers

pub fn add_contribution_tier(
    ctx: Context<AddContributionTier>,
    tier_id: u64,
    amount: u64
) -> Result<()> {
    let project = &mut ctx.accounts.project;
    
    // Add a new contribution tier to the project
    project.contribution_tiers.push(ContributionTier { tier_id, amount });
    
    Ok(())
}

#[derive(Accounts)]
pub struct AddContributionTier<'info> {
    #[account(mut, has_one = owner)]
    pub project: Account<'info, ProjectState>,
    #[account(mut)]
    pub owner: Signer<'info>,
}
