use anchor_lang::prelude::*;
// use anchor_spl::{token::{TokenAccount, Mint, Transfer, Token, transfer, close_account, CloseAccount}, associated_token::AssociatedToken};

mod instructions;
mod state;
use instructions::*;

declare_id!("9swm7FNBS5GxMG6es1yoEYQhNmRjhbNozDp2u1ogybgd");

#[program]
pub mod pixel_game {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize_handler(ctx)
    }

    pub fn attack(ctx: Context<AttackOpponent>, defender: Pubkey) -> Result<()> {
        attack_handler(ctx, defender)
    }
}

#[error_code]
pub enum GameError {
    #[msg("The "player" PubKey on the PlayerStats struct does not match the one signing the txn")]
    Unauthorized,
    #[msg("The "defender" PubKey value in the instruction parameter does not match defenders PlayerStats "player" value.")]
    DefenderError,
    #[msg("The "attacker" has 0 energy left.")]
    InsufficientEnergy,
    #[msg("The "attacker" has 0 energy left.")]
    HealTime,
}
