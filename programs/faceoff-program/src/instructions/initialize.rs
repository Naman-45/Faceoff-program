use anchor_lang::prelude::*;

use crate::state::Challenge;

#[derive(Accounts)]
#[instruction(challenge_id: String)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = creator,
        space = 8 + Challenge::INIT_SPACE,
        seeds = [b"challenge", challenge_id.as_bytes()],
        bump
    )]
    pub challenge: Account<'info, Challenge>,
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        mut,
        seeds = [b"wager_account", challenge_id.as_bytes()],
        bump
    )]
    pub program_account: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn init_challenge(ctx: Context<Initialize>, _challenge_id: String, wager_amount: u64) -> Result<()> {
    let challenge = &mut ctx.accounts.challenge;
        challenge.creator = ctx.accounts.creator.key();
        challenge.wager_amount = wager_amount;
        challenge.opponent = None;
        challenge.result_settled = false;
        challenge.winner = None;
        challenge.challenge_bump = ctx.bumps.challenge;
        challenge.program_account_bump = ctx.bumps.program_account;

        // Transfer SOL from creator to the program account
        let transfer_instruction = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.creator.key(),
            &ctx.accounts.program_account.key(),
            wager_amount,
        );
        anchor_lang::solana_program::program::invoke(
            &transfer_instruction,
            &[
                ctx.accounts.creator.to_account_info(),
                ctx.accounts.program_account.to_account_info(),
            ],
        )?;

        Ok(())
}
