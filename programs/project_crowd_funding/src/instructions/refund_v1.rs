use anchor_lang::prelude::*;
use crate::state::{ProjectState, ProjectStatus};
use crate::errors::CrowdfundingError;

pub fn refund(ctx: Context<Refund>, amount: u64) -> Result<()> {
    let project = &mut ctx.accounts.project;

    require!(project.status == ProjectStatus::Failing, CrowdfundingError::ProjectNotFailing);
    require!(project.current_funding > 0, CrowdfundingError::InsufficientFunds);

    // Create the transfer context
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info().clone(),
        anchor_lang::system_program::Transfer {
            from: ctx.accounts.app_address.to_account_info(), // Using project to transfer funds
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

    /// CHECK:
    #[account(mut)]
    pub contributor: AccountInfo<'info>, // Contributor's wallet

    #[account(mut, signer)]
    pub app_address: Signer<'info>, // Contributor's wallet

    /// CHECK:
    pub system_program: Program<'info, System>, // System program for SOL transfers
}
