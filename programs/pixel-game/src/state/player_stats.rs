use anchor_lang::prelude::*;

#[account]
pub struct PlayerStats {
    pub player: Pubkey,
    pub energy: i64,
    pub health: i64,
    pub attack: i64,
    pub level: i64,
    pub xp_points: i64,
    pub xp_to_next_level: i64,
}

impl PlayerStats {
    pub fn is_owner(&self, account: &AccountInfo) -> bool {
        self.player == *account.key
    }

    pub fn take_damage(&mut self, amount: i64) {
        self.health = self.health.saturating_sub(amount);
    }

        // Method to calculate XP earned from defeating an enemy
        pub fn calculate_xp_gain(&self, enemy_level: i64) -> i64 {
            let base_xp: i64 = 50; // Base XP for defeating an enemy, adjust as necessary
            let level_difference: i64 = enemy_level - self.level;
            let xp_gain: i64 = (base_xp * enemy_level) * (1 + level_difference / 10);
            xp_gain.max(1) // Ensure at least 1 XP is gained
        }

            // Method to Calculate the XP needed for the next level
    pub fn calculate_xp_for_next_level(&self) -> i64 {
        let base_xp_required: i64 = 100; // Base XP required for the first level-up
        let growth_factor: f64 = 1.5; // Determines how much more XP is needed for each subsequent level

        // Calculate the XP for the next level with an increasing difficulty
        ((base_xp_required as f64) * growth_factor.powi(self.level as i32)).round() as i64
    }
}