use crate::state::ProjectStatus;
use crate::state::project::ContributionTier;
use crate::state::project::ProjectState; // Correctly import ProjectState from the project module
use anchor_lang::prelude::*; 
use crate::errors::CrowdfundingError;


pub fn add_contribution_tier(
    ctx: Context<AddContributionTier>,
    tier_id: u64,
    amount: u64,
) -> Result<()> {
    let project = &mut ctx.accounts.project;

    // require!(
    //     ctx.accounts.owner.key() == project.wallet_address,
    //     CrowdfundingError::Unauthorized
    // );

    // Ensure project is in draft status
    require!(
        project.status == ProjectStatus::Draft,
        CrowdfundingError::ProjectNotInDraft
    );

    require!(
        project.contribution_tiers.len() < 5,
        CrowdfundingError::MaxContributionTiersReached
    );

    // Add a new contribution tier to the project
    project
        .contribution_tiers
        .push(ContributionTier { tier_id, amount });

    msg!(
        "Added contribution tier ID: {} with amount: {}",
        tier_id,
        amount
    );

    Ok(())
}

#[derive(Accounts)]
pub struct AddContributionTier<'info> {
    #[account(mut, has_one = owner)]
    pub project: Account<'info, ProjectState>,
    #[account(mut)]
    pub owner: Signer<'info>,
}
