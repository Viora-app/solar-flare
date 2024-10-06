// use anchor_lang::prelude::*;
// 
// #[error_code]
// pub enum CrowdfundingError {
//     #[msg("The project has already reached its hard cap.")]
//     HardCapReached,
// 
//     #[msg("The contribution tier is invalid.")]
//     InvalidContributionTier,
// 
//     #[msg("The contribution amount is less than the minimum tier amount.")]
//     InvalidContributionAmount,
// 
//     #[msg("The project is not published.")]
//     ProjectNotPublished,
// 
//     #[msg("The project deadline has passed.")]
//     DeadlinePassed,
// 
//     #[msg("The contribution amount does not match the required tier amount.")]
//     IncorrectAmount,
// 
//     #[msg("Project is not in draft status.")]
//     ProjectNotInDraft,
// 
//     #[msg("No contribution tiers available.")]
//     NoContributionTiers,
// 
//     // Fixed error message to match the test
//     #[msg("Maximum contribution tiers reached.")]
//     MaxContributionTiersReached,  // Update this to match the test
// 
//     #[msg("You are not authorized to add tiers.")]
//     Unauthorized,
// 
//     #[msg("The project cannot be finalized, as it is not in a finalizable state.")]
//     ProjectNotFinalizable,  // Add this new variant
// 
//     #[msg("Contributor not found for refund.")]
//     ContributorNotFound,  // <--- Add this variant
// 
//     #[msg("Deadline not passed.")]
//     DeadlineNotPassed,
// 	
// 	#[msg("The project is not eligible for reimbursements.")]
//     ProjectNotReimbursable,
// 
//     #[msg("The contributor was not found or has no unreimbursed contributions.")]
//     NoUnreimbursedContributions,
// 
//     #[msg("The project does not have sufficient funds for reimbursement.")]
//     InsufficientFunds,
// 
//     #[msg("The contribution tier was not found.")]
//     TierNotFound,
// }


use anchor_lang::prelude::*;

#[error_code]
pub enum CrowdfundingError {
    #[msg("The project is not in Draft state.")]
    ProjectNotInDraft,

    #[msg("The project is not Published.")]
    ProjectNotPublished,

    #[msg("The project's deadline has passed.")]
    DeadlinePassed,

    #[msg("The project has reached the hard cap.")]
    HardCapReached,

    #[msg("The soft cap has not been reached.")]
    SoftCapNotReached,

    #[msg("The project's deadline has not been reached.")]
    DeadlineNotReached,

    #[msg("The project is not in Failed state.")]
    ProjectNotFailed,

    #[msg("The project must have at least one contribution tier.")]
    NoContributionTiers,

    #[msg("The project already has the maximum number of contribution tiers.")]
    MaxContributionTiersReached,

    #[msg("The contribution tier was not found.")]
    TierNotFound,

    #[msg("The contribution amount does not match the required tier amount.")]
     IncorrectAmount,
	 
	 #[msg("The InsufficientFunds was not found or has no unreimbursed contributions.")]
	 InsufficientFunds
}
