use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TransferChecked, transfer_checked};
use anchor_spl::token::Token;

use crate::errors::CrowdfundingError;
use crate::state::{Project, ProjectStatus};

pub fn contribute_spl_tokens(ctx: Context<Contribute>, amount: u64, tier_id: u64) -> Result<()> {
    let source = &ctx.accounts.contributer_ata;
    let destination = &ctx.accounts.project_ata;
    let token_program = &ctx.accounts.token_program;
    let authority = &ctx.accounts.contributer;
    let mint = &ctx.accounts.usdc_mint;
    let project: &mut Account<'_, Project> = &mut ctx.accounts.project;

    msg!("Contributer: {:?}", ctx.accounts.contributer.key);

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
    
    let transfer_accounts_options = TransferChecked {
        from: source.to_account_info(),
        mint: mint.to_account_info(),
        to: destination.to_account_info(),
        authority: authority.to_account_info(), // Correct signer
    };
    
    let cpi_context = CpiContext::new(token_program.to_account_info(), transfer_accounts_options);
    
    transfer_checked(cpi_context, amount, mint.decimals)?;
    
    
    project.current_funding += amount;
    

    // TODO: Handle tier-specific contributions here

    // Transfer tokens from contributor to the project
    // let cpi_accounts = SplTransfer {
    //     from: source.to_account_info(),
    //     to: destination.to_account_info(),
    //     authority: authority.to_account_info(),
    // };
    // let cpi_program = token_program.to_account_info();
    // anchor_spl::token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;

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
        associated_token::authority = contributer
    )]
    pub contributer_ata: InterfaceAccount<'info, TokenAccount>, // Contributor's associated token account

    #[account(
        init_if_needed,
        payer = contributer,
        associated_token::mint = usdc_mint,
        associated_token::authority = project
    )]
    pub project_ata: InterfaceAccount<'info, TokenAccount>, // Project's associated token account

    pub usdc_mint: InterfaceAccount<'info, Mint>, // USDC Mint
    
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

