use anchor_lang::prelude::*;

#[error_code]
pub enum CrowdfundingError {
    #[msg("The project is not in Draft state.")]
    ProjectNotInDraft,

    #[msg("The project is not Published.")]
    ProjectNotPublished,

    #[msg("The project's deadline has passed.")]
    DeadlinePassed,

    #[msg("The project's deadline Not passed.")]
    DeadlineNotPassed,

    #[msg("The project has reached the hard cap.")]
    HardCapReached,

    #[msg("The soft cap has not been reached.")]
    SoftCapNotReached,

    #[msg("The project's deadline has not been reached.")]
    DeadlineNotReached,

    #[msg("The project is not in Failing state.")]
    ProjectNotFailing,

    #[msg("The project has failed.")]
    ProjectFailed,

    #[msg("The project must have at least one contribution tier.")]
    NoContributionTiers,

    #[msg("The project already has the maximum number of contribution tiers.")]
    MaxContributionTiersReached,

    #[msg("The contribution tier was not found.")]
    TierNotFound,

    #[msg("The contribution amount does not match the required tier amount.")]
    IncorrectAmount,

    #[msg("The project account does not have enough funds.")]
    InsufficientFunds,
}
