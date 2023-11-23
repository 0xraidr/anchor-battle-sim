use anchor_lang::prelude::*;
use crate::{state::PlayerStats, GameError};

pub fn attack_handler(ctx: Context<AttackOpponent>, defender: Pubkey) -> Result<()> {
    // COMMENTING OUT ENERGY DEPLETION FOR TESTING PURPOSES
    // let player_stats = &mut ctx.accounts.players_stats;
    // if player_stats.energy > 0 {
    //     player_stats.energy -= 1;
    // } else {
    //     return Err(GameError::InsufficientEnergy.into());
    // }

    let attackerstat = &mut ctx.accounts.players_stats;
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
        let clock = Clock::get()?;
        let latest_timestamp = clock.unix_timestamp;

        if defenderstat.health <= 0 {
            msg!("Attacker wins!");

///////////////////////////////////////
            // TESTING XP GAIN FUNCTION
    // Calculate XP gain for the attacker
    let xp_gain = attackerstat.calculate_xp_gain(defenderstat.level);
    attackerstat.xp_points += xp_gain;
    msg!("Attacker gains {} XP", xp_gain);
            // TESTING XP GAIN FUNCTION
///////////////////////////////////////
           
            // Set latest healthchange timestamp here so after the battle each players clock can start to count till
            // health regenerates
            defenderstat.last_heal_timestamp = latest_timestamp;
            attackerstat.last_heal_timestamp = latest_timestamp;

            defenderstat.health = 0;
            attackerstat.health = attackerstat.health;
            break;
        }

        attackerstat.take_damage(defenderstat.attack);
        msg!("Attacker took {} damage, remaining health: {}", defenderstat.attack, attackerstat.health);
        
        if attackerstat.health <= 0 {
            msg!("Defender wins!");

///////////////////////////////////////
            // TESTING XP GAIN FUNCTION
    // Calculate XP gain for the attacker
    let xp_gain = attackerstat.calculate_xp_gain(defenderstat.level);
    attackerstat.xp_points += xp_gain;
    msg!("Attacker gains {} XP", xp_gain);
            // TESTING XP GAIN FUNCTION
///////////////////////////////////////

            attackerstat.last_heal_timestamp = latest_timestamp;
            defenderstat.last_heal_timestamp = latest_timestamp;

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
        seeds = [b"players_stats!", attacker.key().as_ref()],
        bump,
        constraint = players_stats.player == *attacker.key, // Ensures the account is owned by the signer
    )]
    pub players_stats: Account<'info,PlayerStats>,
    #[account(mut)]
    pub attacker: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(
        mut, 
        seeds = [b"players_stats!", defender.player.key().as_ref()],
        bump
    )]
    pub defender: Account<'info,PlayerStats>,
}