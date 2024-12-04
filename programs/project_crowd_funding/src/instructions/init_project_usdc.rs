use anchor_lang::prelude::*;
use crate::state::{Project, ProjectStatus};

pub fn init_project(
    ctx: Context<InitProject>,
    project_id: u64,
    soft_cap: u64,
    hard_cap: u64,
    deadline: i64,
    // owner: Pubkey, // Because it's duplicated in the InitProject Context
) -> Result<()> {
    // TODO: Check if project_id isn't already existing in the blockchain 
    // require soft_cap to be lower than hard_cap
    require!(soft_cap < hard_cap, CrowdfundingError::InvalidSoftCap);

    //Convert the owner from Signer to Pubkey
    let owner = ctx.accounts.owner.to_account_info(); // This line is redundant, because it's already in the context as ctx.accounts.owner.Pubkey

    //Create a new Project
    let project = Project {
        project_id,
        owner,
        soft_cap,
        hard_cap,
        deadline,
        current_funding: 0,
        contribution_tiers: Vec::new(),
        status: ProjectStatus::Draft,
    };

    //Create the Project on the blockchain
    ctx.accounts.project.save(&project)?;

    //Initialize the Project in the memory
    *ctx.accounts.project = project;

    msg!("Project initialized with ID: {}", project.project_id);
    Ok(())
}

#[derive(Accounts)]
#[instruction(project_id: u64)]
pub struct InitProject<'info> {
    #[account(init, 
        seeds = [project_id.to_le_bytes().as_ref()], 
        bump, 
        payer = owner, 
        space = 8 + Project::LEN)]
    pub project: Account<'info, Project>,
    #[account(mut)]
    pub owner: Signer<'info>, // The artist or project owner
    pub system_program: Program<'info, System>,
}
