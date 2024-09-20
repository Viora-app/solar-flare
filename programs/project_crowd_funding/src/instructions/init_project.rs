// src/instructions/init_project.rs
use anchor_lang::prelude::*;
use crate::state::project::{ProjectState, ProjectStatus};  // Import the ProjectState from state

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
    project.project_id = project_id;
    project.soft_cap = soft_cap;
    project.hard_cap = hard_cap;
    project.deadline = deadline;
    project.current_funding = 0;
	project.muzikie_address = muzikie_address;
	project.owner = wallet_address;
    project.status = ProjectStatus::Draft;
	
		
	msg!("Project initialized with ID: {}", project.project_id);
    Ok(())
}

#[derive(Accounts)]
#[instruction(project_id: u64)]
pub struct InitProject<'info> {
    #[account(init, seeds = [project_id.to_le_bytes().as_ref()], bump, payer = owner, space = 8 + ProjectState::LEN)]
    pub project: Account<'info, ProjectState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
