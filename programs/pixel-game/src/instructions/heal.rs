use anchor_lang::prelude::*;
use crate::{state::PlayerStats, GameError};
use anchor_lang::system_program::Transfer;
use anchor_lang::system_program::transfer;

pub fn heal_handler(ctx: Context<Heal>, amount: u64) -> Result<()> {
    let player_stats = &mut ctx.accounts.players_stats;
    let latest_health_update = player_stats.last_heal_timestamp;

    if latest_health_update == 100 {
        return Err(GameError::MaxHealth.into())
    } else {
        let accounts = Transfer {
            from: ctx.accounts.signer.to_account_info(),
            to: ctx.accounts.treasury.to_account_info(),
        };
        let cpi = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            accounts,
        );
        transfer(cpi, amount)?;
    // Update player's health and last heal timestamp
    player_stats.health = 100;
    player_stats.last_heal_timestamp = Clock::get()?.unix_timestamp;
    Ok(())
        }
    }



#[derive(Accounts)]
pub struct Heal<'info> {
    #[account(
        mut,
        seeds = [b"players_stats", signer.key().as_ref()],
        bump
    )]
    pub players_stats: Account<'info,PlayerStats>,
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
    )]
    pub treasury: SystemAccount<'info>, 
    pub system_program: Program<'info, System>,
}