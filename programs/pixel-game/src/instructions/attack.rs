use anchor_lang::prelude::*;
use crate::{state::PlayerStats, GameError};

pub fn attack_handler(ctx: Context<AttackOpponent>, defender: Pubkey) -> Result<()> {
    let player_stats = &mut ctx.accounts.player_stats;
    player_stats.energy -= 1;

    let attackerstat = &mut ctx.accounts.player_stats;
    if !attackerstat.is_owner(&ctx.accounts.attacker.to_account_info()) {
        return Err(GameError::Unauthorized.into());
    }

    if defender != ctx.accounts.defender.player {
        return Err(GameError::DefenderError.into());
    }

    let defenderstat = &mut ctx.accounts.defender;

    loop {
        defenderstat.take_damage(attackerstat.attack);
        msg!("Defender took {} damage, remaining health: {}", attackerstat.attack, defenderstat.health);

        if defenderstat.health <= 0 {
            msg!("Attacker wins!");
            defenderstat.health = 0;
            attackerstat.health = attackerstat.health;
            break;
        }

        attackerstat.take_damage(defenderstat.attack);
        msg!("Attacker took {} damage, remaining health: {}", defenderstat.attack, attackerstat.health);
        
        if attackerstat.health <= 0 {
            msg!("Defender wins!");
            attackerstat.health = 0;
            defenderstat.health = defenderstat.health;
            break;
        }
    }
    
    Ok(())
    // in the future implement fun situations such as "Defender/Attacker dodged attack!", or "Critical hit!", "Flash Knockout!"
}

#[derive(Accounts)]
pub struct AttackOpponent<'info> {
    #[account(
        mut, 
        seeds = [b"player_stats", attacker.key().as_ref()],
        bump
    )]
    pub player_stats: Account<'info,PlayerStats>,
    #[account(mut)]
    pub attacker: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(
        mut, 
        seeds = [b"player_stats", defender.player.key().as_ref()],
        bump
    )]
    pub defender: Account<'info,PlayerStats>,
}