use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Token, Mint, Transfer as SplTransfer};
use anchor_spl::token::TokenAccount;

use crate::errors::CrowdfundingError;
use crate::state::{ Project, ProjectStatus};



pub fn contribute_spl_tokens(ctx: Context<ContributeSpl>, amount: u64, tier_id: u64) -> Result<()> {
    let source = &ctx.accounts.contributer_ata;
    let destination = &ctx.accounts.project_ata;
    let token_program = &ctx.accounts.token_program;
    let authority = &ctx.accounts.contributer;
    let project = &mut ctx.accounts.project;

    // Check if the tier ID exists in the project's tiers
    require!(
        project.contribution_tiers.iter().any(|tier| tier.tier_id == tier_id),
        CrowdfundingError::TierNotFound
    );

    // Validate the project's state
    require!(project.status != ProjectStatus::Draft, CrowdfundingError::ProjectNotPublished);
    require!(project.status != ProjectStatus::SoldOut, CrowdfundingError::HardCapReached);
    require!(
        !(project.status == ProjectStatus::Failed || project.status == ProjectStatus::Failing),
        CrowdfundingError::ProjectFailed
    );

    // Ensure the contributor's balance is sufficient
    require!(
        source.amount >= amount,
        CrowdfundingError::InsufficientFunds
    );

    // Ensure the project has not reached its deadline
    let current_timestamp: i64 = Clock::get()?.unix_timestamp;
    require!(
        current_timestamp <= project.deadline,
        CrowdfundingError::DeadlineNotPassed
    );

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

// define an anchor instruction that allows fans to contribute to the artist new campaign by USDC
#[derive(Accounts)]
pub struct ContributeSpl<'info> {
    /// The signer contributing tokens
    #[account(mut)]
    pub contributer: Signer<'info>, // Fan's Wallet

    /// Contributor's Associated Token Account for the specified mint
    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = contributer
    )]
    pub contributer_ata: Account<'info, TokenAccount>, // Fan's ATA

    /// The project state account
    #[account(mut)]
    pub project: Account<'info, Project>, // Project State

    /// Project's Associated Token Account for the specified mint
  #[account(
        init_if_needed,
        associated_token::mint = usdc_mint,
        associated_token::authority = project,
        payer = contributer
    )]
    pub project_ata: Account<'info, TokenAccount>, // Project's ATA

    /// The SPL Token program
    pub token_program: Program<'info, Token>,

    /// The System program (required for associated token creation)
    pub system_program: Program<'info, System>,

    /// The Associated Token program
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// The USDC mint account
    pub usdc_mint: Account<'info, Mint>, // USDC Mint
}

