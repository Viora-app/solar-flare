use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer};
use crate::state::ProjectState;

pub fn finalize_project(ctx: Context<FinalizeProject>) -> Result<()> {
    let project = &ctx.accounts.project; // Immutable borrow

    // Fetch the amount to transfer from the immutable reference
    let amount_to_transfer = project.current_funding * 80 / 100;

    if amount_to_transfer == 0 {
        return Err(error!(ErrorCode::InsufficientFunds));
    }


    // Construct the CPI context for the transfer
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        Transfer {
            from: ctx.accounts.muzikie_address.to_account_info(), // Still using the mutable borrow here
            to: ctx.accounts.owner.to_account_info(),
        },
    );

    // Execute the transfer
    system_program::transfer(cpi_context, amount_to_transfer)?;

    // Now mutate the project state
    let project_mut = &mut ctx.accounts.project; // Mutable borrow here
    project_mut.current_funding = 0; // Reset funding to zero after transfer
    msg!("Successfully transferred {} lamports from project to owner.", amount_to_transfer);
    Ok(())
}

#[derive(Accounts)]
pub struct FinalizeProject<'info> {
    #[account(mut)]
    pub project: Account<'info, ProjectState>,
/// CHECK:
	#[account(mut, signer)]
    pub muzikie_address: Signer<'info>,
	/// CHECK:
    #[account(mut)]
    pub owner: AccountInfo<'info>,

    /// CHECK: This is not dangerous because we are only accessing system instructions.
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("The project does not have sufficient funds for this transfer.")]
    InsufficientFunds,
}
