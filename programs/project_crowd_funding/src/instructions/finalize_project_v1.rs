use anchor_lang::prelude::*;
use crate::state::{ProjectState, ProjectStatus};
use crate::errors::CrowdfundingError;

pub fn finalize_project(ctx: Context<FinalizeProject>) -> Result<()> {
    let project = &mut ctx.accounts.project;

    // Ensure the deadline has passed
    require!(Clock::get()?.unix_timestamp >= project.deadline, CrowdfundingError::DeadlineNotReached);

    // If the soft cap is met, mark the project as Successful
    if project.current_funding >= project.soft_cap {
        project.status = ProjectStatus::Successful;
        ctx.accounts.distribute_funds()?;
        msg!("Project successfully finalized.");
    } else {
        project.status = ProjectStatus::Failed;
        msg!("Project failed to meet the soft cap.");
    }

    Ok(())
}

#[derive(Accounts)]
pub struct FinalizeProject<'info> {
    #[account(mut)]
    pub project: Account<'info, ProjectState>,
    pub system_program: Program<'info, System>, // System program for SOL transfers
    #[account(mut)]
    pub artist: Signer<'info>, // Artist's wallet
     /// CHECK
    #[account(mut)]
    pub muzikie: AccountInfo<'info>, // Muzikie's wallet
}

impl<'info> FinalizeProject<'info> {
    pub fn distribute_funds(&self) -> Result<()> {
        let total_funding = self.project.current_funding;

        // Calculate artist and Muzikie's shares (80% to artist, 20% to Muzikie)
        let artist_share = total_funding * 80 / 100;
        let muzikie_share = total_funding * 20 / 100;

        // Transfer SOL to the artist
        let artist_cpi_context = CpiContext::new(
            self.system_program.to_account_info().clone(), // System program account
            anchor_lang::system_program::Transfer {
                from: self.project.to_account_info(),
                to: self.artist.to_account_info(),
            },
        );

        // Invoke the transfer instruction for the artist
        anchor_lang::system_program::transfer(artist_cpi_context, artist_share)?;

        // Transfer SOL to Muzikie
        let muzikie_cpi_context = CpiContext::new(
            self.system_program.to_account_info().clone(), // System program account
            anchor_lang::system_program::Transfer {
                from: self.project.to_account_info(),
                to: self.muzikie.to_account_info(),
            },
        );

        // Invoke the transfer instruction for Muzikie
        anchor_lang::system_program::transfer(muzikie_cpi_context, muzikie_share)?;

        msg!("Funds distributed to artist and Muzikie.");
        Ok(())
    }
}
