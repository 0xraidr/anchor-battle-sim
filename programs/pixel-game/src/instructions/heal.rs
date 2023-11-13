use anchor_lang::prelude::*;
use crate::{state::PlayerStats, GameError};

pub fn heal_handler(ctx: Context<Heal>) -> Result<()> {
    let player_stats = &mut ctx.accounts.player_stats;
    Ok(())
}


#[derive(Accounts)]
pub struct Heal<'info> {
    #[account(
        payer = signer,
        seeds = [b"player_stats", signer.key().as_ref()],
        bump
    )]
    pub player_stats: Account<'info,PlayerStats>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}