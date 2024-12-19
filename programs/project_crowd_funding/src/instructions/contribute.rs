use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TransferChecked, transfer_checked, TokenInterface};

use crate::errors::CrowdfundingError;
use crate::state::{Project, ProjectStatus};

pub fn contribute_spl_tokens(ctx: Context<ContributeSpl>, amount: u64, tier_id: u64) -> Result<()> {

    msg!("Contributer: {:?}", ctx.accounts.contributer.key);

    // Check if the tier ID exists in the project's tiers
    require!(
        ctx.accounts.project.contribution_tiers
        .iter()
        .any(|tier| tier.tier_id == tier_id),
        CrowdfundingError::TierNotFound
    );

    // Validate the project's state
    require!(ctx.accounts.project.status != ProjectStatus::Draft, 
        CrowdfundingError::ProjectNotPublished);
    require!(ctx.accounts.project.status != ProjectStatus::SoldOut, 
        CrowdfundingError::HardCapReached);
    require!(
        !(ctx.accounts.project.status == ProjectStatus::Failed || 
            ctx.accounts.project.status == ProjectStatus::Failing),
        CrowdfundingError::ProjectFailed
    );

    // Ensure the contributor's balance is sufficient
    require!(
        ctx.accounts.contributer_ata.amount >= amount,
        CrowdfundingError::InsufficientFunds
    );

    // Ensure the project has not reached its deadline
    let current_timestamp: i64 = Clock::get()?.unix_timestamp;
    require!(
        current_timestamp <= ctx.accounts.project.deadline,
        CrowdfundingError::DeadlineNotPassed
    );
    
    let transfer_accounts_options = TransferChecked {
        from: ctx.accounts.contributer_ata.to_account_info(),
        to: ctx.accounts.project_ata.to_account_info(),
        mint: ctx.accounts.usdc_mint.to_account_info(),
        authority: ctx.accounts.contributer.to_account_info(),
    };
    
    let cpi_context = 
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
        transfer_accounts_options,);
    
    transfer_checked(cpi_context, amount, ctx.accounts.usdc_mint.decimals)?;

    ctx.accounts.project.current_funding += amount;
    

    // TODO: Handle tier-specific contributions here

    // Transfer tokens from contributor to the project
    // let cpi_accounts = SplTransfer {
    //     from: ctx.accounts.contributer_ata.to_account_info(),
    //     to: ctx.accounts.project_ata.to_account_info(),
    //     authority: ctx.accounts.contributer.to_account_info(),
    // };
    // let cpi_program = ctx.accounts.token_program.to_account_info();
    // anchor_spl::token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;

    msg!("Successfully contributed {} USDC from fan to the project.", amount);
    Ok(())
}

#[derive(Accounts)]
// #[instruction(project_id: u64)]
pub struct ContributeSpl<'info> {
    #[account(mut)]
    pub project: Account<'info, Project>,

    #[account(
        init_if_needed,
        payer = contributer,
        associated_token::mint = usdc_mint,
        associated_token::authority = project
    )]
    pub project_ata: InterfaceAccount<'info, TokenAccount>, // Project's associated token account

    #[account(mut)]
    pub contributer: Signer<'info>, 

    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = contributer
    )]
    pub contributer_ata: InterfaceAccount<'info, TokenAccount>,

    pub usdc_mint: InterfaceAccount<'info, Mint>, // USDC Mint
    
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

