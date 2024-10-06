use anchor_lang::prelude::*;
use crate::state::{ProjectState, ProjectStatus};
use crate::errors::CrowdfundingError;

pub fn finalize_project(ctx: Context<FinalizeProject>) -> Result<()> {
    let project = &mut ctx.accounts.project;
    let current_timestamp = Clock::get()?.unix_timestamp;

    // Ensure the project deadline has passed
    require!(
        current_timestamp > project.deadline,
        CrowdfundingError::DeadlineNotPassed
    );

    if project.status == ProjectStatus::Successful {
        let artist_share = (project.current_funding * 84) / 100;
        let muzikie_share = (project.current_funding * 15) / 100;
        let escrow_balance = **ctx.accounts.escrow.lamports.borrow();

        require!(
            escrow_balance >= artist_share + muzikie_share,
            CrowdfundingError::InsufficientFunds
        );

        // Transfer funds to artist and Muzikie using the generalized transfer method
        ctx.accounts.transfer_funds(&ctx.accounts.owner, artist_share)?;
        ctx.accounts.transfer_funds(&ctx.accounts.muzikie_address, muzikie_share)?;

        msg!("Project finalized successfully. Funds distributed to artist and Muzikie.");

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
    #[account(mut, signer)]
    pub escrow: Signer<'info>, // Escrow holding the contributions
    /// CHECK:
    #[account(mut)]
    pub owner: AccountInfo<'info>, // Artist's wallet
    /// CHECK:
    #[account(mut)]
    pub muzikie_address: AccountInfo<'info>, // Muzikie's wallet

    pub system_program: Program<'info, System>, // System program for SOL transfers
}

impl<'info> FinalizeProject<'info> {
    fn transfer_funds(&self, recipient: &AccountInfo<'info>, amount: u64) -> Result<()> {
        let transfer_context = CpiContext::new(
            self.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: self.escrow.to_account_info(),
                to: recipient.to_account_info(),
            },
        );
        anchor_lang::system_program::transfer(transfer_context, amount)
    }
}
