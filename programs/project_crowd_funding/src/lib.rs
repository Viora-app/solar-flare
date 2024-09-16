#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("9ig7ysifRGMYBJCrZBNSYBnDeb9G4sCX3ZxRv6CwUBCa");

#[program]
pub mod crowdfunding {
    use super::*;

    // Initialize the project
    pub fn init_project(
        ctx: Context<InitProject>,
        project_id: u64,
        soft_cap: u64,
        hard_cap: u64,
        deadline: i64,
        wallet_address: Pubkey,
        muzikie_address: Pubkey,
    ) -> Result<()> {
        let project = &mut ctx.accounts.project;

        // Set initial values based on inputs
        project.project_id = project_id;
        project.soft_cap = soft_cap;
        project.hard_cap = hard_cap;
        project.deadline = deadline;
        project.wallet_address = wallet_address;
        project.muzikie_address = muzikie_address;

        // Set default internal values
        project.status = ProjectStatus::Draft;
        project.current_funding = 0;
        project.contribution_tiers = Vec::new();
        project.contributions = Vec::new();

        // Logging project initialization
        msg!("Project initialized with ID: {}", project.project_id);
        Ok(())
    }

    // Add a contribution tier to the project
    pub fn add_contribution_tier(
        ctx: Context<AddContributionTier>,
        tier_id: u64,
        amount: u64,
    ) -> Result<()> {
        let project = &mut ctx.accounts.project;

        // Ensure only the owner (wallet_address) can add contribution tiers
        require!(
            ctx.accounts.owner.key() == project.wallet_address,
            ErrorCode::Unauthorized
        );

        // Ensure project is in draft status
        require!(
            project.status == ProjectStatus::Draft,
            ErrorCode::ProjectNotInDraft
        );

        // Ensure no more than 5 tiers can be added
        require!(
            project.contribution_tiers.len() < 5,
            ErrorCode::MaxTiersReached
        );

        // Create new tier and add to the contribution_tiers array
        let new_tier = Tier {
            tier_id,
            amount,
        };
        project.contribution_tiers.push(new_tier);

        msg!("Added contribution tier ID: {} with amount: {}", tier_id, amount);
        Ok(())
    }

    // Set the project status to Live if conditions are met
    pub fn set_live(ctx: Context<SetLive>) -> Result<()> {
        let project = &mut ctx.accounts.project;

        // Ensure the project is in draft status
        require!(
            project.status == ProjectStatus::Draft,
            ErrorCode::ProjectNotInDraft
        );

        // Ensure there is at least one contribution tier
        require!(
            !project.contribution_tiers.is_empty(),
            ErrorCode::NoContributionTiers
        );

        // Set the status to Live
        project.status = ProjectStatus::Live;

        msg!("Project status set to Live.");
        Ok(())
    }

    // Contribute to the project if conditions are met
    pub fn contribute(ctx: Context<Contribute>, tier_id: u64, amount: u64) -> Result<()> {
        let project = &mut ctx.accounts.project;
        let contributor = &ctx.accounts.contributor;

        // Check if the project is Live
        require!(
            project.status == ProjectStatus::Live,
            ErrorCode::ProjectNotLive
        );

        // Ensure deadline has not passed
        require!(
            Clock::get()?.unix_timestamp < project.deadline,
            ErrorCode::DeadlinePassed
        );

        // Ensure hard cap is not met
        require!(
            project.current_funding < project.hard_cap,
            ErrorCode::HardCapMet
        );

        // Find the contribution tier
        let tier = project.contribution_tiers.iter().find(|&tier| tier.tier_id == tier_id);
        require!(
            tier.is_some(),
            ErrorCode::TierNotFound
        );
        let tier = tier.unwrap();

        // Ensure contribution amount matches the tier amount
        require!(
            tier.amount == amount,
            ErrorCode::IncorrectAmount
        );

        // Update the current funding
        project.current_funding += amount;

        // Add a contribution entry
        let contribution = Contribution {
            contribution_tier_id: tier_id,
            sender_address: contributor.key(),
        };
        project.contributions.push(contribution);

        // Check if the hard cap is met
        if project.current_funding >= project.hard_cap {
            project.status = ProjectStatus::Successful;
            msg!("Project status set to Successful.");
        } else {
            // Check if the deadline has passed to update status if needed
            if Clock::get()?.unix_timestamp >= project.deadline {
                if project.current_funding >= project.soft_cap {
                    project.status = ProjectStatus::Successful;
                    msg!("Project status set to Successful.");
                } else {
                    project.status = ProjectStatus::Failed;
                    msg!("Project status set to Failed.");
                }
            }
        }

        msg!("Contribution recorded.");
        Ok(())
    }
}

// Context structs

// The context struct for the init_project function
#[derive(Accounts)]
#[instruction(project_id: u64, soft_cap: u64, hard_cap: u64, deadline: i64, wallet_address: Pubkey, muzikie_address: Pubkey)]
pub struct InitProject<'info> {
    #[account(
        init,
        seeds = [project_id.to_le_bytes().as_ref()],
        bump,
        payer = owner,
        space = 8 + ProjectState::MAX_SIZE + 100 // Add 100 bytes of padding
    )]
    pub project: Account<'info, ProjectState>,
    
    #[account(mut)]
    pub owner: Signer<'info>, // The wallet initializing the project
    pub system_program: Program<'info, System>,
}

// The context struct for the add_contribution_tier function
#[derive(Accounts)]
pub struct AddContributionTier<'info> {
    #[account(mut)]
    pub project: Account<'info, ProjectState>,
    pub owner: Signer<'info>, // The wallet that owns the project
}

// The context struct for the set_live function
#[derive(Accounts)]
pub struct SetLive<'info> {
    #[account(mut)]
    pub project: Account<'info, ProjectState>,
    pub owner: Signer<'info>, // The wallet that owns the project
}

// The context struct for the contribute function
#[derive(Accounts)]
pub struct Contribute<'info> {
    #[account(mut)]
    pub project: Account<'info, ProjectState>,
    pub contributor: Signer<'info>, // The wallet making the contribution
}

// Project state that stores all on-chain data for a project
#[account]
pub struct ProjectState {
    pub project_id: u64,                   // ID from Strapi
    pub soft_cap: u64,                     // Soft cap of the project
    pub hard_cap: u64,                     // Hard cap of the project
    pub deadline: i64,                     // Deadline as UNIX timestamp
    pub wallet_address: Pubkey,            // Owner's wallet address
    pub status: ProjectStatus,             // Project's current status
    pub current_funding: u64,              // Amount raised so far
    pub contribution_tiers: Vec<Tier>,     // List of contribution tiers
    pub muzikie_address: Pubkey,           // Muzikie's wallet address
    pub contributions: Vec<Contribution>,  // List of contributions
}

impl ProjectState {
    pub const MAX_SIZE: usize = 8   // project_id
        + 8                         // soft_cap
        + 8                         // hard_cap
        + 8                         // deadline
        + 32                        // wallet_address (Pubkey is 32 bytes)
        + 1                         // status (enum takes 1 byte if there are fewer than 256 variants)
        + 8                         // current_funding
        + 4 + (5 * (8 + 8))         // contribution_tiers (up to 5 tiers, each with tier_id and amount)
        + 4 + (5 * (8 + 32))        // contributions (up to 5 contributions, each with tier_id and sender_address)
        + 32;                       // muzikie_address
}

// Enum for project statuses
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ProjectStatus {
    Draft,
    Live,
    Successful,
    Failed,
    Final,
}

// Struct for Contribution Tiers
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Tier {
    pub tier_id: u64,    // Contribution tier ID from Strapi
    pub amount: u64,     // Amount for the contribution tier
}

// Struct for Contributions
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Contribution {
    pub contribution_tier_id: u64,  // The tier ID the contributor selected
    pub sender_address: Pubkey,     // Contributor's wallet address
}

// Custom error codes
#[error_code]
pub enum ErrorCode {
    #[msg("Project is not in draft status.")]
    ProjectNotInDraft,
    #[msg("Maximum contribution tiers reached.")]
    MaxTiersReached,
    #[msg("You are not authorized to add tiers.")]
    Unauthorized,
    #[msg("No contribution tiers available.")]
    NoContributionTiers,
    #[msg("Deadline has passed.")]
    DeadlinePassed,
    #[msg("Hard cap already met.")]
    HardCapMet,
    #[msg("Contribution amount does not match the tier amount.")]
    IncorrectAmount,
    #[msg("Contribution tier not found.")]
    TierNotFound,
    #[msg("Project is not live.")]
    ProjectNotLive,
}
