use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

// use state::*;
use crate::errors::CrowdfundingError;
use instructions::*;
use state::ProjectState;

// use crate::instruction::AddContributionTier;

declare_id!("31Yra1Eyy4TcU4saGyWRqHamgvEeauVF1gAnjwqArwoW");

#[program]
pub mod crowdfunding {
    use state::ProjectStatus;

    use super::*;

    pub fn init_project(
        ctx: Context<InitProject>, // Ensure `InitProject` is correctly imported
        project_id: u64,
        soft_cap: u64,
        hard_cap: u64,
        deadline: i64,
        wallet_address: Pubkey,
        muzikie_address: Pubkey,
    ) -> Result<()> {
        instructions::init_project::init_project(
            ctx,
            project_id,
            soft_cap,
            hard_cap,
            deadline,
            wallet_address,
            muzikie_address,
        )
    }

    pub fn add_contribution_tier(
        ctx: Context<AddContributionTier>, // Ensure `AddContributionTier` is correctly imported
        tier_id: u64,
        amount: u64,
    ) -> Result<()> {
        instructions::add_tier::add_contribution_tier(ctx, tier_id, amount)
    }

    pub fn set_publish(ctx: Context<SetPublish>) -> Result<()> {
        let project = &mut ctx.accounts.project;

        // Ensure the project is in draft status
        require!(
            project.status == ProjectStatus::Draft,
            CrowdfundingError::ProjectNotInDraft
        );

        // Ensure there is at least one contribution tier
        require!(
            !project.contribution_tiers.is_empty(),
            CrowdfundingError::NoContributionTiers
        );

        // Set the status to Published
        project.status = ProjectStatus::Published;

        msg!("Project status set to Published.");
        Ok(())
    }

    pub fn contribute(
        ctx: Context<Contribute>, // Ensure `Contribute` is correctly imported
        tier_id: u64,
        amount: u64,
    ) -> Result<()> {
        instructions::contribute::contribute(ctx, tier_id, amount)
    }

    // pub fn finalize(
    //     ctx: Context<Finalize>
    // ) -> Result<()> {
    //     instructions::finalize::finalize(ctx)
    // }
}

// The context struct for the set_live function
#[derive(Accounts)]
pub struct SetPublish<'info> {
    #[account(mut)]
    pub project: Account<'info, ProjectState>,
    pub owner: Signer<'info>, // The wallet that owns the project
}
