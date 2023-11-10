use anchor_lang::prelude::*;
use crate::{state::PlayerStats, 
    // GameError
};

pub fn initialize_handler(ctx: Context<Initialize>) -> Result<()> {
    let player_stats = &mut ctx.accounts.player_stats;
    player_stats.player = ctx.accounts.signer.key();
    player_stats.health = 100;
    player_stats.energy = 10;
    player_stats.attack = 25;
    player_stats.energy = 3;
    player_stats.level = 0;
    player_stats.xp_points = 0;
    player_stats.xp_to_next_level = 75;
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + 32 + 8 + 8 + 8 + 8 + 8 + 8,
        seeds = [b"player_stats", signer.key().as_ref()],
        bump
    )]
    pub player_stats: Account<'info,PlayerStats>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}