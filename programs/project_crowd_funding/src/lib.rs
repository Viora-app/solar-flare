use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;

// use state::*;
use crate::errors::CrowdfundingError;
use instructions::*;
use state::{ProjectState, ProjectStatus};

declare_id!("31Yra1Eyy4TcU4saGyWRqHamgvEeauVF1gAnjwqArwoW");

#[program]
pub mod crowdfunding {
    use super::*;

    pub fn init_project(
        ctx: Context<InitProject>,
        project_id: u64,
        soft_cap: u64,
        hard_cap: u64,
        deadline: i64,
        wallet_address: Pubkey,
        muzikie_address: Pubkey,
    ) -> Result<()> {
        instructions::init_project_v1::init_project(
            ctx,
            project_id,
            soft_cap,
            hard_cap,
            deadline,
            wallet_address,
            muzikie_address,
        )
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

    pub fn contribute(ctx: Context<Contribute>, amount: u64, tier_id: u64) -> Result<()> {
        instructions::contribute_v1::contribute(ctx, amount, tier_id)
    }

    pub fn finalize_project(ctx: Context<FinalizeProject>) -> Result<()> {
        instructions::finalize_project_v1::finalize_project(ctx)
    }

    pub fn refund(ctx: Context<Refund>, amount: u64) -> Result<()> {
        instructions::refund_v1::refund(ctx, amount)
    }
}



// The context struct for the set_live function
#[derive(Accounts)]
pub struct SetPublish<'info> {
    #[account(mut)]
    pub project: Account<'info, ProjectState>,
    pub owner: Signer<'info>, // The wallet that owns the project
}
