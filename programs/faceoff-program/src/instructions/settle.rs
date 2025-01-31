use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use crate::{error::CustomError, state::Challenge,};

#[derive(Accounts)]
#[instruction(challenge_id: String)]
pub struct Settle<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"challenge", challenge_id.as_bytes()],
        bump
    )]
    pub challenge: Account<'info, Challenge>,

    #[account(
        mut,
        seeds = [b"wager_account", challenge_id.as_bytes()],
        bump
    )]
    pub program_account: SystemAccount<'info>,
    pub system_program: Program<'info, System>
}

pub fn settle_challenge<'a, 'b, 'c, 'info>(ctx: Context<'a, 'b, 'c, 'info, Settle<'info>>, winner: Option<Pubkey>, challenge_id: String) -> Result<()> {
    let challenge = &mut ctx.accounts.challenge;
    let remaining_accounts = &ctx.remaining_accounts;

    require!(remaining_accounts.len() >= 2, CustomError::MissingAccounts);
    
    let creator_account = &remaining_accounts[0];
    let opponent_account = &remaining_accounts[1];

    require!(!challenge.result_settled, CustomError::WagerAlreadySettled);
    require!(remaining_accounts.len() >= 2, CustomError::MissingAccounts);

    if let Some(winner_pubkey) = winner {
        require!(
            challenge.creator == winner_pubkey || 
            challenge.opponent.map_or(false, |o| o == winner_pubkey),
            CustomError::ThirdPersonWinner
        );
    }

    challenge.result_settled = true;
    challenge.winner = winner;

    let signer_seeds: &[&[&[u8]]] = &[&[b"wager_account", challenge_id.as_ref(), &[challenge.program_account_bump]]];

    match winner {
        Some(winner_pubkey) => {
            let payout = challenge.wager_amount * 2;
            let winner_account = if winner_pubkey == challenge.creator {
                creator_account.to_account_info()
            } else {
                opponent_account.to_account_info()
            };
    
            let program_account = ctx.accounts.program_account.to_account_info();
    
            let transfer_accounts = Transfer {
                from: program_account,
                to: winner_account,
            };
    
            let transfer_ctx = CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                transfer_accounts,
                signer_seeds,
            );
    
            transfer(transfer_ctx, payout)?;
        }
        None => {
            let program_account = ctx.accounts.program_account.to_account_info();
            let system_program = ctx.accounts.system_program.to_account_info();
    
            let transfer_to_creator = Transfer {
                from: program_account.clone(), 
                to: creator_account.to_account_info(),
            };
    
            let transfer_to_opponent = Transfer {
                from: program_account, 
                to: opponent_account.to_account_info(),
            };
    
            let creator_transfer_ctx = CpiContext::new_with_signer(
                system_program.clone(),
                transfer_to_creator,
                signer_seeds,
            );
    
            let opponent_transfer_ctx = CpiContext::new_with_signer(
                system_program,
                transfer_to_opponent,
                signer_seeds,
            );
    
            transfer(creator_transfer_ctx, challenge.wager_amount)?;
            transfer(opponent_transfer_ctx, challenge.wager_amount)?;
        }
    }
    Ok(())
}
