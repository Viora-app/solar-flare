use anchor_lang::prelude::*;
use crate::state::{Project, ProjectStatus};
use crate::errors::CrowdfundingError;

pub fn init_project(
    ctx: Context<InitProject>,
    project_id: u64,
    soft_cap: u64,
    hard_cap: u64,
    deadline: i64,
) -> Result<()> {
    // TODO: Check if project_id isn't already existing in the blockchain 
    // require soft_cap to be lower than hard_cap
    require!(soft_cap < hard_cap, CrowdfundingError::InvalidSoftCap);

   

    //Create a new Project
    let project = &mut ctx.accounts.project;
    project.project_id = project_id;
    project.soft_cap = soft_cap;
    project.hard_cap = hard_cap;
    project.deadline = deadline;
    project.current_funding = 0;
    project.contribution_tiers = Vec::new();
    project.status = ProjectStatus::Draft;
   
    msg!("Project initialized with ID: ");
    Ok(())
}

#[derive(Accounts)]
#[instruction(project_id: u64)]
pub struct InitProject<'info> {
    #[account(init, 
        seeds = [project_id.to_le_bytes().as_ref()], 
        bump, 
        payer = artist, 
        space = 8 + Project::INIT_SPACE)]
    pub project: Account<'info, Project>,
    #[account(mut)]
    pub artist: Signer<'info>, // The artist or project owner
    pub system_program: Program<'info, System>,
}
