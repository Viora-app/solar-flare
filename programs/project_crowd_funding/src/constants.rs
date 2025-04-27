use anchor_lang::prelude::*;
#[account]
pub struct VioraAccount {
    pub total_earnings: u64,      // Accumulated earnings (15% share)
    pub bump: u8,                 // Bump for PDA verification
}