use anchor_lang::prelude::*;
use anchor_lang::system_program::Transfer;
use anchor_lang::system_program::transfer;
use crate::state::project::{ProjectState , ProjectStatus, Contribution}; // Correctly import ProjectState from the project module
use crate::errors::CrowdfundingError;

pub fn contribute(
    ctx: Context<Contribute>,
    tier_id: u64,
    amount: u64
) -> Result<()> {
    let project = &mut ctx.accounts.project;
    let contributor = &ctx.accounts.contributor;

    // Check if the project is live
    require!(
        project.status == ProjectStatus::Live,
        CrowdfundingError::ProjectNotLive
    );

    // Ensure the deadline has not passed
    require!(
        Clock::get()?.unix_timestamp < project.deadline,
        CrowdfundingError::DeadlinePassed
    );

    // Ensure the hard cap is not met
    require!(
        project.current_funding < project.hard_cap,
        CrowdfundingError::HardCapReached
    );

    // Find the contribution tier
    let tier = project.contribution_tiers.iter().find(|&tier| tier.tier_id == tier_id);
    require!(
        tier.is_some(),
        CrowdfundingError::TierNotFound
    );
    let tier = tier.unwrap();

    // Ensure the contribution amount matches the tier amount
    require!(
        tier.amount == amount,
        CrowdfundingError::IncorrectAmount
    );

    // Update the current funding
    project.current_funding += amount;

    // Add a contribution entry
    let contribution = Contribution {
        contribution_tier_id: tier_id,
        sender_address: contributor.key(),
    };
    project.contributions.push(contribution);

    // Check if the hard cap is met
    if project.current_funding >= project.hard_cap {
        project.status = ProjectStatus::Successful;
        msg!("Project status set to Successful.");
    } else {
        // Check if the deadline has passed to update status if needed
        if Clock::get()?.unix_timestamp >= project.deadline {
            if project.current_funding >= project.soft_cap {
                project.status = ProjectStatus::Successful;
                msg!("Project status set to Successful.");
            } else {
                project.status = ProjectStatus::Failed;
                msg!("Project status set to Failed.");
            }
        }
    }
    
    // Transfer SOL from contributor to the project
    ctx.accounts.transfer_sol(amount)?;

    msg!("Contribution recorded.");
    Ok(())
}

#[derive(Accounts)]
pub struct Contribute<'info> {
    #[account(mut)]
    pub project: Account<'info, ProjectState>,
    #[account(mut)]
    pub contributor: Signer<'info>, // The wallet making the contribution
    pub system_program: Program<'info, System>,  // System program for SOL transfers
}

impl<'info> Contribute<'info> {
    pub fn transfer_sol(&self, amount: u64) -> Result<()> {
        let contributor = &self.contributor;
        let project_account = &self.project;

        // Prepare the transfer instruction
        let transfer_instruction = Transfer {
            from: contributor.to_account_info(),
            to: project_account.to_account_info(),
        };

        // Create the CPI context for transferring SOL
        let transfer_ctx = CpiContext::new(self.system_program.to_account_info(), transfer_instruction);

        // Perform the SOL transfer using the system program
        transfer(transfer_ctx, amount)?;

        msg!("Transferred {} lamports from contributor to project", amount);
        Ok(())
    }
}
