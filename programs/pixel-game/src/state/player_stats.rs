use anchor_lang::prelude::*;
// use crate::GameError;


#[account]
pub struct PlayerStats {
    pub player: Pubkey,
    pub energy: i64,
    pub health: i64,
    pub attack: i64,
    pub level: i64,
    pub xp_points: i64,
    pub xp_to_next_level: i64,
    pub last_heal_timestamp: i64,
}

impl PlayerStats {
    pub const LEN: usize = 8 + 32 + 8 * 7;

    pub fn is_owner(&self, account: &AccountInfo) -> bool {
        self.player == *account.key
    }

    pub fn take_damage(&mut self, amount: i64) {
        self.health = self.health.saturating_sub(amount);
    }

    pub fn calculate_xp_gain(&self, enemy_level: i64) -> i64 {
        let base_xp: i64 = 50; // Base XP for defeating an enemy
        let level_multiplier: i64 = 2; // Multiplier based on enemy's level
        let level_difference: i64 = enemy_level - self.level;
        let difficulty_modifier: i64 = (level_difference / 2).max(1); // Modifier based on level difference

        // Calculate the XP gain
        (base_xp + (enemy_level * level_multiplier)) * difficulty_modifier
    }

            // Method to Calculate the XP needed for the next level
    pub fn calculate_xp_for_next_level(&self) -> i64 {
        let base_xp_required: i64 = 100; // Base XP required for the first level-up
        let growth_factor: f64 = 1.5; // Determines how much more XP is needed for each subsequent level

        // Calculate the XP for the next level with an increasing difficulty
        ((base_xp_required as f64) * growth_factor.powi(self.level as i32)).round() as i64
    }

    //     // This is for the health instruction
    // pub fn heal(&mut self) -> Result<()> {

    //     Ok(())
    // }
}