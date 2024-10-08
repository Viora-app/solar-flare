use anchor_lang::prelude::*;
use crate::state::project_v1::ContributionTier;
use crate::state::ProjectState;
use crate::errors::CrowdfundingError;

pub fn add_tier(ctx: Context<AddTier>, tier_id: u64, amount: u64) -> Result<()> {
    let project = &mut ctx.accounts.project;

    // Ensure we have fewer than 5 tiers
    require!(project.contribution_tiers.len() < 5, CrowdfundingError::MaxContributionTiersReached);

    project.contribution_tiers.push(ContributionTier { tier_id, amount });

    msg!("Added contribution tier ID: {} with amount: {}", tier_id, amount);

    Ok(())
}

#[derive(Accounts)]
pub struct AddTier<'info> {
    #[account(mut)]
    pub project: Account<'info, ProjectState>,

    #[account(mut, signer)]
    pub owner: Signer<'info>, // Project owner
}
