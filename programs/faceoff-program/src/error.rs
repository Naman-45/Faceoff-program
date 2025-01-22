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
}
