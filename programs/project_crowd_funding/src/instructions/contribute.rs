use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, Transfer as SplTransfer};
use anchor_spl::token::TokenAccount;
use crate::errors::CrowdfundingError;
use crate::state::{ Project, ProjectStatus};



pub fn contribute_spl_tokens(ctx: Context<ContributeSpl>, amount: u64, tier_id: u64) -> Result<()> {
    
    let source = &ctx.accounts.contributer_ata;
    let destination = &ctx.accounts.project_ata;
    let token_program = &ctx.accounts.token_program;
    let authority = &ctx.accounts.contributer;
    let project = &mut ctx.accounts.project;

    // Check the Tier_id is exists in the project tier
    require!(project.contribution_tiers.iter().any(|tier| tier.tier_id == tier_id), 
        CrowdfundingError::TierNotFound);

    /* validate project state */
    // Check if project is still active, not sold out, and not in a failed state
    require!(project.status != ProjectStatus::Draft, 
        CrowdfundingError::ProjectNotPublished);
    require!(project.status != ProjectStatus::SoldOut, 
        CrowdfundingError::HardCapReached);
    require!(!(project.status == ProjectStatus::Failed 
        || project.status == ProjectStatus::Failing), 
        CrowdfundingError::ProjectFailed);

    // Ensure the contributer's balance is sufficient
    require!(
        source.amount >= amount,
        CrowdfundingError::InsufficientFunds
    );
    // Ensure the project has not reached its deadline
    let current_timestamp: i64 = Clock::get()?.unix_timestamp;
    require!(
        current_timestamp > project.deadline,
        CrowdfundingError::DeadlineNotPassed
    );

    //TODO: Make use of TierId into the contributions

    // Transfer tokens from Fan to Project
    // Create references to the destination, source, token_program, and authority from the context
    // This struct will provide account information when making a cross-program invocation (CPI) to the SPL Token program
    let cpi_accounts = SplTransfer {
        from: source.to_account_info(),
        to: destination.to_account_info(),
        authority: authority.to_account_info(),
    };
    let cpi_program = token_program.to_account_info();
    token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;
    msg!("Successfully contributed {} USDC from fan to the project.", amount);
    Ok(())
}

// define an anchor instruction that allows fans to contribute to the artist new campaign by USDC
#[derive(Accounts)]
pub struct ContributeSpl<'info> {
    #[account(mut)]
    pub contributer: Signer<'info>, // From Fan's Wallet
    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = contributer
    )]
    pub contributer_ata: Account<'info, TokenAccount>, // The Fan's ATA
    #[account(mut)]
    pub project: Account<'info, Project>,
    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = project
    )]
    pub project_ata: Account<'info, TokenAccount>, //The Project (Campaign)'s ATA
    pub token_program: Program<'info, Token>, //the token program
    pub usdc_mint: Account<'info, Mint>, // USDC mint account
}