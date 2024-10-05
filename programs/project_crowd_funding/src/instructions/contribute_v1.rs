use crate::errors::CrowdfundingError;
use crate::state::{ProjectState, ProjectStatus};
use anchor_lang::prelude::*;

pub fn contribute(ctx: Context<Contribute>, amount: u64, tier_id: u64) -> Result<()> {
    let project = &mut ctx.accounts.project;

    require!(
        project.status == ProjectStatus::Published,
        CrowdfundingError::ProjectNotPublished
    );
    require!(
        Clock::get()?.unix_timestamp < project.deadline,
        CrowdfundingError::DeadlinePassed
    );

    let tier = project.contribution_tiers.iter().find(|&t| t.tier_id == tier_id);
    require!(tier.is_some(), CrowdfundingError::TierNotFound);
    let tier = tier.unwrap();
    require!(tier.amount == amount, CrowdfundingError::IncorrectAmount);

    require!(
        project.current_funding + amount <= project.hard_cap,
        CrowdfundingError::HardCapReached
    );

    project.current_funding += amount;

    // Check if the project has reached the soft or hard cap
    if project.current_funding >= project.hard_cap {
        project.status = ProjectStatus::SoldOut;
        msg!("Project has reached the hard cap and is SoldOut.");
    } else if project.current_funding >= project.soft_cap {
        project.status = ProjectStatus::Successful;
        msg!("Project has reached the soft cap and is Successful.");
    }

    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info().clone(),
        anchor_lang::system_program::Transfer {
            from: ctx.accounts.contributor.to_account_info(),
            to: ctx.accounts.project.to_account_info(),
        },
    );

    // Invoke the transfer instruction
    anchor_lang::system_program::transfer(cpi_context, amount)?;

    msg!("Contribution of {} lamports recorded.", amount);
    Ok(())
}

#[derive(Accounts)]
pub struct Contribute<'info> {
    #[account(mut)]
    pub project: Account<'info, ProjectState>,
    #[account(mut)]
    pub contributor: Signer<'info>, // Wallet making the contribution
    pub system_program: Program<'info, System>, // System program for SOL transfers
}
