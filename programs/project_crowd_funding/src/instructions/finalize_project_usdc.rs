use anchor_lang::prelude::*;
use crate::state::{ ProjectStatus. Project};
use crate::errors::CrowdfundingError;

pub fn finalize_project(ctx: Context<FinalizeProject>) -> Result<()> {
    let project = &mut ctx.accounts.project;
    let current_timestamp = Clock::get()?.unix_timestamp;
    let artist_share = 85;
    let viora_share = 15;
    // Ensure the project deadline has passed
    // We have to disable this check for DEMO
    require!(
        current_timestamp > project.deadline,
        CrowdfundingError::DeadlineNotPassed
    );

    if project.status == ProjectStatus::Successful || project.status == ProjectStatus::SoldOut {
        let artist_share = (project.current_funding * artist_share) / 100; // TODO: check if this division operator works properly

        // Transfer funds to artist and App using the generalized transfer method
        ctx.accounts.transfer_funds(&ctx.accounts.owner, artist_share)?;
        // TODO: Transfer viora share
        ctx.accounts.transfer_funds(&ctx.accounts.viora, (project.current_funding * viora_share) / 100)?;
        msg!("Project finalized successfully. Funds distributed to artist and app.");

        // let cpi_accounts = SplTransfer {
        //     from: source.to_account_info(),
        //     to: destination.to_account_info(),
        //     authority: authority.to_account_info(),
        // };
        // let cpi_program = token_program.to_account_info();
        // token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;
        // msg!("Successfully contributed {} USDC from fan {} to the project {}.", amount, from, to);
        // Ok(())


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
