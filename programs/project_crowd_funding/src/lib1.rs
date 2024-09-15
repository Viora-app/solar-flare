use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer};
use anchor_lang::solana_program::system_instruction;

declare_id!("YourProgramID");

#[program]
pub mod crowdfunding {
    use super::*;

    // Create a project with softcap, hardcap, deadline, and title
    pub fn create_project(
        ctx: Context<CreateProject>, 
        softcap: u64, 
        hardcap: u64, 
        deadline: i64, 
        title: String
    ) -> Result<()> {
        let project = &mut ctx.accounts.project;

        // Assign project details
        project.artist = ctx.accounts.artist.key();
        project.softcap = softcap;
        project.hardcap = hardcap;
        project.deadline = deadline;
        project.current_funding = 0;
        project.title = title;

        Ok(())
    }

    // Contribute to a project
    pub fn contribute(ctx: Context<Contribute>, amount: u64) -> Result<()> {
        let project = &mut ctx.accounts.project;

        // Transfer the contribution amount to the project's PDA
        let ix = system_instruction::transfer(
            &ctx.accounts.contributor.key(),
            &ctx.accounts.project.key(),
            amount,
        );
        invoke(
            &ix,
            &[
                ctx.accounts.contributor.to_account_info(),
                ctx.accounts.project.to_account_info(),
            ],
        )?;

        // Update project funding
        project.current_funding += amount;

        Ok(())
    }

    // Withdraw funds if the project reached the softcap
    pub fn withdraw_funds(ctx: Context<WithdrawFunds>) -> Result<()> {
        let project = &mut ctx.accounts.project;

        // Transfer all the current funding to the artist
        let ix = system_instruction::transfer(
            &ctx.accounts.project.key(),
            &ctx.accounts.artist.key(),
            project.current_funding,
        );
        invoke(
            &ix,
            &[
                ctx.accounts.project.to_account_info(),
                ctx.accounts.artist.to_account_info(),
            ],
        )?;

        Ok(())
    }

    // Refund contributors if the softcap wasn't reached
    pub fn refund_contributors(ctx: Context<RefundContributors>, amount: u64) -> Result<()> {
        let project = &mut ctx.accounts.project;

        // Transfer the specified amount back to the contributor
        let ix = system_instruction::transfer(
            &ctx.accounts.project.key(),
            &ctx.accounts.contributor.key(),
            amount,
        );
        invoke(
            &ix,
            &[
                ctx.accounts.project.to_account_info(),
                ctx.accounts.contributor.to_account_info(),
            ],
        )?;

        Ok(())
    }
}

// Accounts for creating a project
#[derive(Accounts)]
pub struct CreateProject<'info> {
    #[account(
        init,
        payer = artist,
        space = 8 + 32 + 40 + 8 + 8 + 8 + 8,
        seeds = [artist.key().as_ref(), title.as_bytes()],
        bump
    )]
    pub project: Account<'info, Project>,
    #[account(mut)]
    pub artist: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Accounts for contributing to a project
#[derive(Accounts)]
pub struct Contribute<'info> {
    #[account(mut)]
    pub project: Account<'info, Project>,
    #[account(mut)]
    pub contributor: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Accounts for withdrawing funds
#[derive(Accounts)]
pub struct WithdrawFunds<'info> {
    #[account(mut)]
    pub project: Account<'info, Project>,
    #[account(mut)]
    pub artist: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Accounts for refunding contributors
#[derive(Accounts)]
pub struct RefundContributors<'info> {
    #[account(mut)]
    pub project: Account<'info, Project>,
    #[account(mut)]
    pub contributor: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Data structure for a project
#[account]
pub struct Project {
    pub artist: Pubkey,
    pub title: String,
    pub softcap: u64,
    pub hardcap: u64,
    pub deadline: i64,
    pub current_funding: u64,
}
