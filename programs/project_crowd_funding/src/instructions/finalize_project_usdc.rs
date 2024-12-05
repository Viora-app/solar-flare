use anchor_lang::prelude::*;
use crate::state::{ ProjectStatus. Project};
use crate::errors::CrowdfundingError;

pub fn finalize_project(ctx: Context<FinalizeProject>) -> Result<()> {
    let project = &mut ctx.accounts.project; // Maybe project status be modified
    let project_ata = &mut ctx.accounts.project_ata;
    let artist_ata = &mut ctx.accounts.artist_ata;
    let viora_ata = &mut ctx.accounts.viora_ata;
    let current_timestamp = Clock::get()?.unix_timestamp;
    let artist_share = 0.85;
    let viora_share = 0.15;
    // Ensure the project deadline has passed
    // We have to disable this check for DEMO
    require!(
        current_timestamp > project.deadline,
        CrowdfundingError::DeadlineNotPassed
    );

    if project.status == ProjectStatus::Successful || project.status == ProjectStatus::SoldOut {
        let artist_amount = project.current_funding * artist_share; // TODO: check if this division operator works properly
        let viora_amount = project.current_funding * viora_share;
        // Transfer funds to artist and App using the generalized transfer method
        // ctx.accounts.transfer_funds(&ctx.accounts.owner, artist_share)?;
        // // TODO: Transfer viora share
        // ctx.accounts.transfer_funds(&ctx.accounts.viora, (project.current_funding * viora_share) / 100)?;
        // msg!("Project finalized successfully. Funds distributed to artist and app.");

        // Transfer Artist share
        let cpi_accounts = SplTransfer {
            from: project_ata.to_account_info(),
            to: artist_ata.to_account_info(),
            authority: project.to_account_info(),
        };
        let cpi_program = token_program.to_account_info();
        token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;
        msg!("Successfully transfered {} USDC as artist share from project_ata {} to the artist_ata {}.", amount, from, to);


        // Transfer Viora share
        let cpi_accounts = SplTransfer {
            from: project_ata.to_account_info(),
            to: viora_ata.to_account_info(),
            authority: project.to_account_info(),
        };
        let cpi_program = token_program.to_account_info();
        token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;
        msg!("Successfully transfered {} USDC as platform share from project_ata {} to the viora_ata {}.", amount, from, to);


        Ok(())


    } else if project.status == ProjectStatus::Published {
        project.status = ProjectStatus::Failing;
        msg!("Project failed to reach the soft cap and is marked as failing.");
    }

    Ok(())
}

#[derive(Accounts)]
pub struct FinalizeProject<'info> {
    #[account(mut)]
    pub project: AccountInfo<'info>, // Project
    #[account(mut)]
    pub project_ata: Account<'info, TokenAccount>, // The Project's ATA

    pub artist: Account<'info, Project>
    #[account(mut)]
    pub artist_ata: Account<'info, TokenAccount>, //The Artist's ATA
    pub viora: Account<'info, Project>
    #[account(mut)]
    pub viora_ata: Account<'info, TokenAccount>, //The Viora's ATA
    pub system_program: Program<'info, System>, // System program for SPL transfers
}