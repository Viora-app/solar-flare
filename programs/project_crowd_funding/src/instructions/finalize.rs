// use anchor_lang::prelude::*;
// use anchor_lang::system_program::{transfer, Transfer};
// use crate::state::{ProjectState, ProjectStatus,}; 
// use crate::errors::CrowdfundingError;
// 
// // use anchor_lang::prelude::*;
// // use crate::state::{ProjectState, ProjectStatus, Contribution};
// // use crate::errors::CrowdfundingError;
// 
// pub fn finalize(ctx: Context<Finalize>) -> Result<()> {
//     let project = &mut ctx.accounts.project;
// 
//     // Ensure the deadline has passed
//     require!(
//         Clock::get()?.unix_timestamp >= project.deadline,
//         CrowdfundingError::DeadlinePassed
//     );
// 
//     // Ensure the project is either successful or failed
//     require!(
//         project.status == ProjectStatus::Successful || project.status == ProjectStatus::Failed,
//         CrowdfundingError::ProjectNotFinalizable
//     );
// 
//     if project.status == ProjectStatus::Successful {
//         // Project was successful, distribute funds
// 
//         let total_funds = project.current_funding;
//         let wallet_share = total_funds * 90 / 100;
//         let muzikie_share = total_funds * 10 / 100;
// 
//         // Transfer 90% to the project owner (wallet address)
//         **project.to_account_info().try_borrow_mut_lamports()? -= wallet_share;
//         **ctx.accounts.wallet_address.try_borrow_mut_lamports()? += wallet_share;
// 
//         // Transfer 10% to Muzikie
//         **project.to_account_info().try_borrow_mut_lamports()? -= muzikie_share;
//         **ctx.accounts.muzikie_address.try_borrow_mut_lamports()? += muzikie_share;
// 
//         msg!(
//             "Finalized project: {} lamports to wallet, {} to Muzikie.",
//             wallet_share,
//             muzikie_share
//         );
// 
//     } else if project.status == ProjectStatus::Failed {
//         // Refund contributors if the project failed
//         for contribution in project.contributions.iter() {
//             // Fetch the contribution tier to get the amount
//             let tier = project.contribution_tiers.iter().find(|&tier| tier.tier_id == contribution.contribution_tier_id);
//             require!(tier.is_some(), CrowdfundingError::TierNotFound);
// 
//             let refund_amount = tier.unwrap().amount;
// 
//             // Refund each contributor (minus fee, if applicable)
//             **project.to_account_info().try_borrow_mut_lamports()? -= refund_amount;
//             let contributor_account = ctx.remaining_accounts.iter().find(|account| account.key() == &contribution.sender_address);
//             if let Some(contributor_info) = contributor_account {
//                 **contributor_info.try_borrow_mut_lamports()? += refund_amount;
//                 msg!("Refunded {} lamports to {}", refund_amount, contribution.sender_address);
//             } else {
//                 return Err(CrowdfundingError::ContributorNotFound.into());
//             }
//         }
//     }
// 
//     // Set the status to Final
//     project.status = ProjectStatus::Final;
//     msg!("Project finalized.");
//     Ok(())
// }
// 
// #[derive(Accounts)]
// pub struct Finalize<'info> {
//     #[account(mut)]
//     pub project: Account<'info, ProjectState>,
//     #[account(mut)]
//     pub wallet_address: Signer<'info>,  // The wallet address of the project owner
//     #[account(mut)]
//     pub muzikie_address: Signer<'info>, // Muzikie wallet address
//     // The remaining accounts will include contributors that need to be refunded
// }
// 
// 
// #[derive(Accounts)]
// pub struct FinalizeProject<'info> {
//     #[account(mut)]
//     pub project: Account<'info, ProjectState>,
//     #[account(mut)]
//     pub owner: Signer<'info>,  // The owner of the project
//     #[account(mut)]
//     pub wallet_address: AccountInfo<'info>,  // Project owner's wallet
//     #[account(mut)]
//     pub muzikie_address: AccountInfo<'info>, // Muzikie's wallet
//     pub system_program: Program<'info, System>, // System program
// }
// 
// // impl<'info> FinalizeProject<'info> {
// //     pub fn transfer_sol(&self, amount: u64, to: Pubkey) -> Result<()> {
// //         let project_account = &self.project;
// // 
// //         // Prepare the transfer instruction
// //         let transfer_instruction = Transfer {
// //             from: project_account.to_account_info(),
// //             to,
// //         };
// // 
// //         // Create the CPI context for transferring SOL
// //         let transfer_ctx = CpiContext::new(self.system_program.to_account_info(), transfer_instruction);
// // 
// //         // Perform the SOL transfer using the system program
// //         transfer(transfer_ctx, amount)?;
// // 
// //         msg!("Transferred {} lamports", amount);
// //         Ok(())
// //     }
// // }
// 
// // #[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
// // pub struct Contribution {
// //     pub contribution_tier_id: u64,
// //     pub sender_address: Pubkey, // Store the contributor's address
// // }
