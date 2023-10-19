use anchor_lang::prelude::*;
use anchor_spl::{token::{TokenAccount, Mint, Transfer, Token, transfer, close_account, CloseAccount}, associated_token::AssociatedToken};


declare_id!("3UukPvWro2LyhZWX7rLEkKD6U8jTH1AUeq6w18FKUe8W");

#[program]
pub mod pixel_game {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let player_stats = &mut ctx.accounts.player_stats;
        player_stats.player = ctx.accounts.signer.key();
        player_stats.health = 100;
        player_stats.energy = 100;
        player_stats.attack = 25;
        Ok(())
    }

    pub fn attack(ctx: Context<AttackOpponent>) -> Result<()> {
        let updated_attacker_stats = &mut ctx.accounts.updated_attacker_stats;
        updated_attacker_stats.player = ctx.accounts.attacker.key();

        // have to create the logic for attacking so that i can create the variables value below.
        // new_attacker_stats.health = attacker_taken_dmg;
        let energy = &mut updated_attacker_stats.energy;

        *energy -= 1;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_if_needed,
        payer = signer,
        space = 500,
        seeds = [b"stats", signer.key().as_ref()],
        bump
    )]
    pub player_stats: Account<'info,PlayerStats>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AttackOpponent<'info> {
    #[account(
        seeds = [b"stats", attacker.key().as_ref()],
        bump
    )]
    pub updated_attacker_stats: Account<'info,PlayerStats>,
    #[account(mut)]
    pub attacker: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub opponent: SystemAccount<'info>,
    pub opponent_token: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = opponent_token,
        associated_token::authority = opponent
    )]
    pub maker_ata: Account<'info, TokenAccount>,
    #[account(
        seeds = [b"opstats", opponent.key().as_ref()],
        bump
    )]
    pub opponent_stats: Account<'info,PlayerStats>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,

}

#[account]
pub struct PlayerStats {
    pub player: Pubkey,
    pub energy: i64,
    pub health: i64,
    pub attack: i64,
}
