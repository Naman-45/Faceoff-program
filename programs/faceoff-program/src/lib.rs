pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

use instructions::*;

declare_id!("J7NDrWGG2Y77xq21VphX1ue4xeEX4LViLAPheNhni8cb");

#[program]
pub mod faceoff_program {
    use super::*;

    pub fn create_challenge(ctx: Context<Initialize>, challenge_id: String, wager_amount:u64) -> Result<()> {
        initialize::init_challenge(ctx, challenge_id, wager_amount)?;
        Ok(())
    }

    pub fn join_challenge(ctx: Context<JoinChallenge>, challenge_id: String, wager_amount:u64) -> Result<()> {
        join::join_challenge(ctx, challenge_id, wager_amount)?;
        Ok(())
    }

    pub fn settle_wager(ctx: Context<Settle>, winner: Option<Pubkey>, challenge_id: String) -> Result<()> {
        settle::settle_challenge(ctx, winner, challenge_id)?;
        Ok(())
    }
}
