use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use crate::state::project::{ProjectState, ProjectStatus}; 
use crate::errors::CrowdfundingError;

pub fn reimburse(ctx: Context<Reimburse>) -> Result<()> {
    let project = &mut ctx.accounts.project;
    let reimburser = &ctx.accounts.reimburser;

    // Ensure the project is in Published or Failed state
    require!(
        project.status == ProjectStatus::Published || project.status == ProjectStatus::Failed,
        CrowdfundingError::ProjectNotReimbursable
    );

    // Phase 1: Collect refund information (immutable borrow)
    let mut total_refund_amount: u64 = 0;
    let mut contributions_to_reimburse = vec![];
    let mut found_unreimbursed_contribution = false;

    for contribution in project.contributions.iter() {
        if contribution.sender_address == reimburser.key() && !contribution.reimbursed {
            // Find the contribution tier
            let tier = project
                .contribution_tiers
                .iter()
                .find(|&tier| tier.tier_id == contribution.contribution_tier_id);
            require!(tier.is_some(), CrowdfundingError::TierNotFound);

            let refund_amount = tier.unwrap().amount;

            // Mark this contribution for reimbursement
            total_refund_amount += refund_amount;
            contributions_to_reimburse.push(contribution.contribution_tier_id);
            found_unreimbursed_contribution = true;
        }
    }

    // If no unreimbursed contributions were found, throw an error
    require!(found_unreimbursed_contribution, CrowdfundingError::NoUnreimbursedContributions);

    // Phase 2: Mutate the state (mutable borrow)
    // Ensure there are enough funds in the project to cover the refund
    require!(project.current_funding >= total_refund_amount, CrowdfundingError::InsufficientFunds);

    // Update the contributions and mark them as reimbursed
    for contribution in project.contributions.iter_mut() {
        if contributions_to_reimburse.contains(&contribution.contribution_tier_id) {
            contribution.reimbursed = true;
        }
    }

    // Subtract the total refund amount from the project's current funding
    project.current_funding -= total_refund_amount;

    // Perform the refund using CPI (Cross-Program Invocation)
    let cpi_accounts = Transfer {
        from: project.to_account_info(),
        to: reimburser.to_account_info(),
    };
    let cpi_context = CpiContext::new(ctx.accounts.system_program.to_account_info(), cpi_accounts);
    transfer(cpi_context, total_refund_amount)?;

    msg!("Refunded {} lamports to {}", total_refund_amount, reimburser.key());

    // Set the status to Reimbursing
    project.status = ProjectStatus::Reimbursing;

    // If all funds are reimbursed, set the status to Final
    if project.current_funding == 0 {
        project.status = ProjectStatus::Final;
        msg!("All funds reimbursed. Project status set to Final.");
    }

    Ok(())
}

#[derive(Accounts)]
pub struct Reimburse<'info> {
    #[account(mut)]
    pub project: Account<'info, ProjectState>,
    #[account(mut)]
    pub reimburser: Signer<'info>,  // The contributor requesting the refund
    pub system_program: Program<'info, System>,  // System program for SOL transfers
}
