use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount};
use anchor_spl::token::{Token, Transfer as SplTransfer};

// use crate::errors::CrowdfundingError;
use crate::state::Project;

pub fn contribute_spl_tokens(ctx: Context<Contribute>, amount: u64, tier_id: u64) -> Result<()> {
    let source = &ctx.accounts.contributer_ata;
    let destination = &ctx.accounts.project_ata;
    let token_program = &ctx.accounts.token_program;
    let authority = &ctx.accounts.contributer;
    // let project: &mut Account<'_, Project> = &mut ctx.accounts.project;
// 
//     msg!("Contributer: {:?}", ctx.accounts.contributer.key);
// 
//     // Check if the tier ID exists in the project's tiers
//     require!(
//         project.contribution_tiers.iter().any(|tier| tier.tier_id == tier_id),
//         CrowdfundingError::TierNotFound
//     );

    // Validate the project's state
    // require!(project.status != ProjectStatus::Draft, CrowdfundingError::ProjectNotPublished);
    // require!(project.status != ProjectStatus::SoldOut, CrowdfundingError::HardCapReached);
    // require!(
    //     !(project.status == ProjectStatus::Failed || project.status == ProjectStatus::Failing),
    //     CrowdfundingError::ProjectFailed
    // );

    // // Ensure the contributor's balance is sufficient
    // require!(
    //     source.amount >= amount,
    //     CrowdfundingError::InsufficientFunds
    // );

    // Ensure the project has not reached its deadline
    // let current_timestamp: i64 = Clock::get()?.unix_timestamp;
    // require!(
    //     current_timestamp <= project.deadline,
    //     CrowdfundingError::DeadlineNotPassed
    // );

    // TODO: Handle tier-specific contributions here

    // Transfer tokens from contributor to the project
    let cpi_accounts = SplTransfer {
        from: source.to_account_info(),
        to: destination.to_account_info(),
        authority: authority.to_account_info(),
    };
    let cpi_program = token_program.to_account_info();
    anchor_spl::token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;

    msg!("Successfully contributed {} USDC from fan to the project.", amount);
    Ok(())
}

#[derive(Accounts)]
#[instruction(project_id: u64)]
pub struct Contribute<'info> {
    #[account(mut, signer)]
    pub contributer: Signer<'info>, 

    #[account(mut)]
    pub project: Account<'info, Project>,

    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = contributer,
        constraint = contributer_ata.owner == contributer.key(),
        constraint = contributer_ata.mint == usdc_mint.key()
    )]
    pub contributer_ata: InterfaceAccount<'info, TokenAccount>, // Contributor's associated token account

    #[account(
        init,
        payer = contributer,
        associated_token::mint = usdc_mint,
        associated_token::authority = project,
        constraint = project_ata.owner == token_program.key(),
        constraint = project_ata.mint == usdc_mint.key()
    )]
    pub project_ata: InterfaceAccount<'info, TokenAccount>, // Project's associated token account

    pub usdc_mint: InterfaceAccount<'info, Mint>, // USDC Mint
    
    pub associated_token_program: Program<'info, AssociatedToken>, 
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,

}

