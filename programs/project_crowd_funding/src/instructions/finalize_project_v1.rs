use anchor_lang::prelude::*;
use crate::state::{ProjectState, ProjectStatus};
use crate::errors::CrowdfundingError;

pub fn finalize_project(ctx: Context<FinalizeProject>) -> Result<()> {
    let project = &mut ctx.accounts.project;
    let current_timestamp = Clock::get()?.unix_timestamp;
    let owner_share = 85;

    // Ensure the project deadline has passed
    // We have to disable this check for DEMO
    require!(
        current_timestamp > project.deadline,
        CrowdfundingError::DeadlineNotPassed
    );

    if project.status == ProjectStatus::Successful || project.status == ProjectStatus::SoldOut {
        let artist_share = (project.current_funding * owner_share) / 100;

        // Transfer funds to artist and App using the generalized transfer method
        ctx.accounts.transfer_funds(&ctx.accounts.owner, artist_share)?;

        msg!("Project finalized successfully. Funds distributed to artist and app.");

    } else if project.status == ProjectStatus::Published {
        project.status = ProjectStatus::Failing;
        msg!("Project failed to reach the soft cap and is marked as failing.");
    }

    Ok(())
}

#[derive(Accounts)]
pub struct FinalizeProject<'info> {
    #[account(mut)]
    pub project: Account<'info, ProjectState>,

    /// CHECK:
    #[account(mut)]
    pub owner: AccountInfo<'info>, // Artist's wallet

    /// CHECK:
    #[account(mut, signer)]
    pub app_address: Signer<'info>, // App's wallet

    pub system_program: Program<'info, System>, // System program for SOL transfers
}

impl<'info> FinalizeProject<'info> {
    fn transfer_funds(&self, recipient: &AccountInfo<'info>, amount: u64) -> Result<()> {
        let transfer_context = CpiContext::new(
            self.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: self.app_address.to_account_info(),
                to: recipient.to_account_info(),
            },
        );
        anchor_lang::system_program::transfer(transfer_context, amount)
    }
}
