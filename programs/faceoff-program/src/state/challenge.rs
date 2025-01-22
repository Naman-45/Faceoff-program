use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Challenge {
    pub creator: Pubkey,
    pub opponent: Option<Pubkey>,
    pub wager_amount: u64,
    pub result_settled: bool,
    pub winner: Option<Pubkey>,
    pub challenge_bump: u8,
    pub program_account_bump: u8
}
