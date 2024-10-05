use anchor_lang::prelude::*;
use crate::state::{ProjectState, ProjectStatus};
use crate::errors::CrowdfundingError;

pub fn refund(ctx: Context<Refund>, amount: u64) -> Result<()> {
    let project = &mut ctx.accounts.project;

    // Ensure the project is in Failed state
    require!(project.status == ProjectStatus::Failed, CrowdfundingError::ProjectNotFailed);

    // Create the transfer context
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info().clone(), // System program account
        anchor_lang::system_program::Transfer {
            from: project.to_account_info(),
            to: ctx.accounts.contributor.to_account_info(),
        },
    );

    // Invoke the transfer instruction
    anchor_lang::system_program::transfer(cpi_context, amount)?;

    msg!("Refund of {} lamports issued to contributor.", amount);
    Ok(())
}

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub project: Account<'info, ProjectState>,
    #[account(mut)]
    pub contributor: Signer<'info>, // Contributor's wallet
    pub system_program: Program<'info, System>, // System program for SOL transfers
}
