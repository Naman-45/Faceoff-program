use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("The challenge has already been joined.")]
    ChallengeAlreadyJoined,
    #[msg("You cannot join your own challenge.")]
    CannotJoinYourOwnChallenge,
    #[msg("The wager has already been settled.")]
    WagerAlreadySettled,
    #[msg("The wager amount is incorrect.")]
    IncorrectWagerAmount,
    #[msg("Winner is neither creator nor opponent.")]
    ThirdPersonWinner,
    #[msg("Missing accounts in remaining_accounts")]
    MissingAccounts
}
