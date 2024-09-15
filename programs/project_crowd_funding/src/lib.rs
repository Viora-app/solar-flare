#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("3mDN8aNgwdM6BUBDiaxbmpRSwWoLeyrMtak5fbLsWtkW");

#[program]
pub mod crowdfunding {
    use super::*;

    pub fn create_project(ctx: Context<CreateProject>, title: String) -> Result<()> {
        let project = &mut ctx.accounts.project;

        // Convert title to a fixed-length Vec<u8> (20 bytes max)
        let mut title_bytes = title.as_bytes().to_vec();
        title_bytes.resize(20, 0); // Ensure the title is exactly 20 bytes long (pad with zeros if shorter)

        project.artist = ctx.accounts.artist.key();
        project.title = title_bytes;
        project.project_id = Clock::get()?.unix_timestamp as u64; // Generate unique ID using timestamp
        project.current_funding = 0; // Initialize funding

        // Log a message to confirm the creation
        msg!("Project created by artist {}", project.artist);

        Ok(())
    }

    pub fn contribute(ctx: Context<Contribute>, amount: u64) -> Result<()> {
        let project = &mut ctx.accounts.project;

        // Log the contribution
        msg!(
            "Contributor {} is contributing {} lamports",
            ctx.accounts.contributor.key(),
            amount
        );

        // Update project's funding
        project.current_funding = project
            .current_funding
            .checked_add(amount)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        // Transfer the contribution amount to the project PDA account
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.contributor.key(),
            &ctx.accounts.project.key(),
            amount,
        );

        // Execute the instruction
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.contributor.to_account_info(),
                ctx.accounts.project.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        Ok(())
    }
}

#[account]
pub struct ProjectState {
    pub artist: Pubkey,           // 32 bytes
    pub title: Vec<u8>,           // Fixed length, 20 bytes
    pub project_id: u64,          // 8 bytes
    pub current_funding: u64,     // 8 bytes
}

// In the account creation, space should be adjusted properly:
#[derive(Accounts)]
#[instruction(title: String)]
pub struct CreateProject<'info> {
    #[account(
        init,
        seeds = [title.as_bytes(), artist.key().as_ref()],
        bump,
        payer = artist,
        space = 8 + 32 + 4 + 20 + 8 + 8  // 8 for discriminator, 32 for artist, 4 for Vec<u8> (prefix length), 20 for title, 8 for project_id, 8 for current_funding
    )]
    pub project: Account<'info, ProjectState>,
    #[account(mut)]
    pub artist: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Contribute<'info> {
    #[account(mut)]
    pub project: Account<'info, ProjectState>,
    #[account(mut)]
    pub contributor: Signer<'info>,
    pub system_program: Program<'info, System>,
}
