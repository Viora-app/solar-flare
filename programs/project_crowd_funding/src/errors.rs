use anchor_lang::prelude::*;

#[error_code]
pub enum CrowdfundingError {
    #[msg("The project has already reached its hard cap.")]
    HardCapReached,

    #[msg("The contribution tier is invalid.")]
    InvalidContributionTier,

    #[msg("The contribution amount is less than the minimum tier amount.")]
    InvalidContributionAmount,
    #[msg("The project is not live.")]
    ProjectNotLive,

    #[msg("The project deadline has passed.")]
    DeadlinePassed,

    #[msg("The contribution tier was not found.")]
    TierNotFound,

    #[msg("The contribution amount does not match the required tier amount.")]
    IncorrectAmount,

    #[msg("Project is not in draft status.")]
    ProjectNotInDraft,

    #[msg("No contribution tiers available.")]
    NoContributionTiers,
}
