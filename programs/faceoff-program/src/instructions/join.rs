use anchor_lang::prelude::*;

use crate::{error::CustomError, state::Challenge};

#[derive(Accounts)]
#[instruction(challenge_id: String)]
pub struct JoinChallenge<'info> {
    #[account(
        mut,
        seeds = [b"challenge", challenge_id.as_bytes()],
        bump
    )]
    pub challenge: Account<'info, Challenge>,
    #[account(mut)]
    pub opponent: Signer<'info>,
    #[account(
        mut,
        seeds = [b"wager_account", challenge_id.as_bytes()],
        bump
    )]
    pub program_account: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn join_challenge(ctx: Context<JoinChallenge>, _challenge_id: String, wager_amount: u64) -> Result<()> {
    let challenge = &mut ctx.accounts.challenge;

    require!(
        challenge.opponent.is_none(),
        CustomError::ChallengeAlreadyJoined
    );

    require!(
        challenge.creator != ctx.accounts.opponent.key(),
        CustomError::CannotJoinYourOwnChallenge
    );

    require!(
        challenge.wager_amount == wager_amount,
        CustomError::IncorrectWagerAmount
    );

    challenge.opponent = Some(ctx.accounts.opponent.key());

    // Transfer SOL from opponent to the program account
    let transfer_instruction = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.opponent.key(),
        &ctx.accounts.program_account.key(),
        wager_amount,
    );
    anchor_lang::solana_program::program::invoke(
        &transfer_instruction,
        &[
            ctx.accounts.opponent.to_account_info(),
            ctx.accounts.program_account.to_account_info(),
        ],
    )?;

    Ok(())
}