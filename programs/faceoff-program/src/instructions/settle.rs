use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use crate::{error::CustomError, state::Challenge};

#[derive(Accounts)]
#[instruction(challenge_id: String)]
pub struct Settle<'info> {
    #[account(
        mut,
        seeds = [b"challenge", challenge_id.as_bytes()],
        bump
    )]
    pub challenge: Account<'info, Challenge>,
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(mut)]
    pub opponent: Signer<'info>,
    #[account(
        mut,
        seeds = [b"wager_account", challenge_id.as_bytes()],
        bump
    )]
    pub program_account: SystemAccount<'info>,
    pub system_program: Program<'info, System>
}

pub fn settle_challenge(ctx: Context<Settle>, winner: Option<Pubkey>, challenge_id: String) -> Result<()> {
    let challenge = &mut ctx.accounts.challenge;

    require!(!challenge.result_settled, CustomError::WagerAlreadySettled);

    challenge.result_settled = true;
    challenge.winner = winner;

    let signer_seeds: &[&[&[u8]]] = &[&[b"wager_account", challenge_id.as_ref(), &[challenge.program_account_bump]]];

    match winner {
        Some(winner_pubkey) => {
            // Transfer total wager amount to the winner
            let payout = challenge.wager_amount * 2;
            let winner_account = if winner_pubkey == challenge.creator {
                ctx.accounts.creator.to_account_info()
            } else {
                ctx.accounts.opponent.to_account_info()
            };

            transfer(
                CpiContext::new_with_signer(
                    ctx.accounts.system_program.to_account_info(),
                    Transfer {
                        from: ctx.accounts.program_account.to_account_info(),
                        to: winner_account,
                    },
                    signer_seeds,
                ),
                payout,
            )?;

        }
        None => {
            transfer(
                CpiContext::new_with_signer(
                    ctx.accounts.system_program.to_account_info(),
                    Transfer {
                        from: ctx.accounts.program_account.to_account_info(),
                        to: ctx.accounts.creator.to_account_info(),
                    },
                    signer_seeds,
                ),
                challenge.wager_amount,
            )?;

            transfer(
                CpiContext::new_with_signer(
                    ctx.accounts.system_program.to_account_info(),
                    Transfer {
                        from: ctx.accounts.program_account.to_account_info(),
                        to: ctx.accounts.opponent.to_account_info(),
                    },
                    signer_seeds,
                ),
                challenge.wager_amount,
            )?;
        }
    }

    Ok(())
}
