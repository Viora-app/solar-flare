use anchor_lang::prelude::*;
use crate::state::{ProjectState, ProjectStatus};
use crate::errors::CrowdfundingError;


pub fn contribute(ctx: Context<Contribute>, tier_id: u64, amount: u64) -> Result<()> {
    let project = &mut ctx.accounts.project;

    require!(project.status != ProjectStatus::Draft, CrowdfundingError::ProjectNotPublished);
    require!(project.status != ProjectStatus::SoldOut, CrowdfundingError::HardCapReached);
    require!(!(project.status == ProjectStatus::Failed || project.status == ProjectStatus::Failing), CrowdfundingError::ProjectFailed);


    // Find the contribution tier
    let tier = project.contribution_tiers.iter().find(|&t| t.tier_id == tier_id);
    require!(tier.is_some(), CrowdfundingError::TierNotFound);

    let tier = tier.unwrap();
    require!(tier.amount == amount, CrowdfundingError::IncorrectAmount);

    project.current_funding += amount;

    if project.current_funding >= project.hard_cap {
        project.status = ProjectStatus::SoldOut;
        msg!("Project has reached the hard cap and is sold out.");
    } else if project.current_funding >= project.soft_cap {
        project.status = ProjectStatus::Successful;
        msg!("Project has reached the soft cap and is Successful.");
    }

    // Transfer SOL from the contributor to the project's account
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info().clone(),
        anchor_lang::system_program::Transfer {
            from: ctx.accounts.contributor.to_account_info(),
            to: ctx.accounts.app_address.to_account_info(),
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

    #[account(mut, signer)]
    pub contributor: Signer<'info>, 

    /// CHECK
    #[account(mut)]
    pub app_address: AccountInfo<'info>, 

    pub system_program: Program<'info, System>, 
}
