use anchor_lang::prelude::*;
// use anchor_spl::{token::{TokenAccount, Mint, Transfer, Token, transfer, close_account, CloseAccount}, associated_token::AssociatedToken};


declare_id!("GK1fv7iaZFijE9YreSqoLy35CVUuBGLeKrmrniBfVT1C");

#[program]
pub mod pixel_game {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let player_stats = &mut ctx.accounts.player_stats;
        player_stats.player = ctx.accounts.signer.key();
        player_stats.health = 100;
        player_stats.energy = 10;
        player_stats.attack = 25;
        Ok(())
    }

    pub fn attack(ctx: Context<AttackOpponent>, defender: Pubkey) -> Result<()> {

        let attackerstat = &mut ctx.accounts.attackerstat;
    
        if !attackerstat.is_owner(&ctx.accounts.attacker.to_account_info()) {
            return Err(GameError::Unauthorized.into());
        }
    
        if defender != ctx.accounts.defender.player {
            return Err(GameError::DefenderError.into());
        }
    
        let defenderstat = &mut ctx.accounts.defender;
    
        loop {
            defenderstat.take_damage(attackerstat.attack);
    
            if defenderstat.health <= 0 {
                msg!("Attacker wins!");
                break;
            }
    
            attackerstat.take_damage(defenderstat.attack);
    
            if attackerstat.health <= 0 {
                msg!("Defender wins!");
                break;
            }
        }
        Ok(())
        // in the future implement fun situations such as "Defender/Attacker dodged attack!", or "Critical hit!", "Flash Knockout!"
    }
}


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + 32 + 64 + 64 + 64,
        seeds = [b"player_stats", signer.key().as_ref()],
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
        seeds = [b"attackerstat", attacker.key().as_ref()],
        bump
    )]
    pub attackerstat: Account<'info,PlayerStats>,
    #[account(mut)]
    pub attacker: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub defender: Account<'info,PlayerStats>,

}

#[account]
pub struct PlayerStats {
    pub player: Pubkey,
    pub energy: i64,
    pub health: i64,
    pub attack: i64,
}

impl PlayerStats {
    pub fn is_owner(&self, account: &AccountInfo) -> bool {
        self.player == *account.key
    }

    pub fn take_damage(&mut self, amount: i64) {
        self.health = self.health.saturating_sub(amount);
    }
}

#[error_code]
pub enum GameError {
    #[msg("The "player" PubKey on the PlayerStats struct does not match the one signing the txn")]
    Unauthorized,
    #[msg("The "defender" PubKey value in the instruction parameter does not match defenders PlayerStats "player" value.")]
    DefenderError
}
