use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, system_instruction};

declare_id!("5LoKChz3AivF2RSwn1QfNMcSmFDSZv44yVZjd2ihyMni");

#[program]
pub mod crowdfunding {
    use super::*;

    // Initialize a crowdfunding project
    pub fn initialize_project(
        ctx: Context<InitializeProject>,
        softcap: u64,
        hardcap: u64,
        timeline: i64, // Unix timestamp
        tiers: Vec<Tier>,
    ) -> Result<()> {
        let project = &mut ctx.accounts.project;
        project.artist = *ctx.accounts.artist.key;
        project.softcap = softcap;
        project.hardcap = hardcap;
        project.timeline = timeline;
        project.tiers = tiers;
        project.total_funds = 0;
        project.finalized = false;
        project.refunded = false;
        Ok(())
    }

    // Contribute to the project
    pub fn contribute(ctx: Context<Contribute>, tier_index: u8) -> Result<()> {
        // Immutable borrow first
        let project_key = ctx.accounts.project.to_account_info().key(); // Get project key immutably
        let project_account_info = ctx.accounts.project.to_account_info(); // Get project account info immutably

        // Now we mutably borrow the project account
        let project = &mut ctx.accounts.project;
        let contribution_amount = project.tiers[tier_index as usize].amount;

        // Ensure contribution is within timeline and doesn't exceed hardcap
        let current_time = Clock::get()?.unix_timestamp;
        require!(current_time <= project.timeline, CustomError::ProjectEnded);
        require!(project.total_funds + contribution_amount <= project.hardcap, CustomError::HardcapExceeded);

        // Transfer funds using the previously stored immutable data
        invoke(
            &system_instruction::transfer(
                &ctx.accounts.funder.key(), // From funder
                &project_key,               // To project account
                contribution_amount,
            ),
            &[
                ctx.accounts.funder.to_account_info(), // Funder account
                project_account_info,                  // Project account info
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        // Update project state mutably after the transfer
        project.total_funds += contribution_amount;
        project.contributions.push(Contribution {
            funder: *ctx.accounts.funder.key,
            amount: contribution_amount,
        });

        Ok(())
    }

    // Finalize project and distribute funds
    pub fn finalize_project(ctx: Context<FinalizeProject>) -> Result<()> {
        // Immutable borrow first
        let project_key = ctx.accounts.project.to_account_info().key(); // Get project key immutably
        let project_account_info = ctx.accounts.project.to_account_info(); // Get project account info immutably

        // Now mutably borrow project
        let project = &mut ctx.accounts.project;

        // Ensure project timeline has ended and softcap is met
        let current_time = Clock::get()?.unix_timestamp;
        require!(current_time >= project.timeline, CustomError::ProjectNotEnded);
        require!(project.total_funds >= project.softcap, CustomError::SoftcapNotMet);

        // Calculate distribution (95% to artist, 5% to platform)
        let artist_amount = (project.total_funds * 95) / 100;
        let platform_amount = project.total_funds - artist_amount;

        // Transfer to artist
        invoke(
            &system_instruction::transfer(
                &project_key,
                &ctx.accounts.artist.key(),
                artist_amount,
            ),
            &[
                project_account_info.clone(), // Use `clone` if you need to reuse the account
                ctx.accounts.artist.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        // Transfer to platform
        invoke(
            &system_instruction::transfer(
                &project_key,
                &ctx.accounts.platform.key(),
                platform_amount,
            ),
            &[
                project_account_info, // Already cloned above, can use here
                ctx.accounts.platform.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        project.finalized = true;
        Ok(())
    }

    // Refund contributions if softcap is not met
    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        // Immutable borrow first
        let project_key = ctx.accounts.project.to_account_info().key();
        let project_account_info = ctx.accounts.project.to_account_info();

        // Now mutably borrow the project
        let project = &mut ctx.accounts.project;

        // Ensure softcap is not met and project has ended
        let current_time = Clock::get()?.unix_timestamp;
        require!(current_time >= project.timeline, CustomError::ProjectNotEnded);
        require!(project.total_funds < project.softcap, CustomError::SoftcapMet);

        // Refund all contributions
        for contribution in &project.contributions {
            invoke(
                &system_instruction::transfer(
                    &project_key, // From project
                    &contribution.funder, // To each funder
                    contribution.amount,
                ),
                &[
                    project_account_info.clone(), // Use cloned project account info
                    ctx.accounts.system_program.to_account_info(),
                ],
            )?;
        }

        project.refunded = true;
        Ok(())
    }
}

// Define the data structures used in the program
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Tier {
    pub amount: u64,
    pub description: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Contribution {
    pub funder: Pubkey,
    pub amount: u64,
}

// Define the state of the project
#[account]
pub struct Project {
    pub artist: Pubkey,
    pub softcap: u64,
    pub hardcap: u64,
    pub timeline: i64,
    pub tiers: Vec<Tier>,
    pub total_funds: u64,
    pub contributions: Vec<Contribution>,
    pub finalized: bool,
    pub refunded: bool,
}

// Define the contexts for each instruction
#[derive(Accounts)]
pub struct InitializeProject<'info> {
    #[account(init, payer = artist, space = 9000)]
    pub project: Account<'info, Project>,
    #[account(mut)]
    pub artist: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Contribute<'info> {
    #[account(mut)]
    pub project: Account<'info, Project>,
    #[account(mut)]
    pub funder: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FinalizeProject<'info> {
    #[account(mut, has_one = artist)]
    pub project: Account<'info, Project>,
    #[account(mut)]
    pub artist: Signer<'info>,
    #[account(mut)]
    pub platform: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub project: Account<'info, Project>,
    pub system_program: Program<'info, System>,
}

// Custom error messages
#[error_code]
pub enum CustomError {
    #[msg("The project has already ended.")]
    ProjectEnded,
    #[msg("The project's hardcap has been exceeded.")]
    HardcapExceeded,
    #[msg("The project's softcap has not been met.")]
    SoftcapNotMet,
    #[msg("The project has not ended yet.")]
    ProjectNotEnded,
    #[msg("The project's softcap has been met.")]
    SoftcapMet,
}
