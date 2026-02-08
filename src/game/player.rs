use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub level: u32,
    pub xp: u64,
    pub xp_to_next: u64,
    pub gp: u64,

    // Base stats (before upgrades/relics)
    pub base_luck: f64,
    pub base_speed: f64,
    pub base_gp_multiplier: f64,
    pub base_xp_multiplier: f64,
    pub base_crit_chance: f64,

    // Computed stats (after upgrades/relics)
    pub luck: f64,
    pub speed: f64,
    pub gp_multiplier: f64,
    pub xp_multiplier: f64,
    pub crit_chance: f64,
    pub auto_speed: f64,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            level: 1,
            xp: 0,
            xp_to_next: 100,
            gp: 0,
            base_luck: 0.0,
            base_speed: 1.0,
            base_gp_multiplier: 1.0,
            base_xp_multiplier: 1.0,
            base_crit_chance: 0.05,
            luck: 0.0,
            speed: 1.0,
            gp_multiplier: 1.0,
            xp_multiplier: 1.0,
            crit_chance: 0.05,
            auto_speed: 0.0,
        }
    }
}

impl Player {
    pub fn recalculate_stats(
        &mut self,
        upgrade_luck: f64,
        upgrade_speed: f64,
        upgrade_gp_mult: f64,
        upgrade_xp_mult: f64,
        upgrade_crit: f64,
        upgrade_auto: f64,
        relic_luck: f64,
        relic_speed_pct: f64,
        relic_gp_mult_pct: f64,
        relic_xp_mult_pct: f64,
        relic_crit: f64,
    ) {
        self.luck = self.base_luck + upgrade_luck + relic_luck;
        self.speed = (self.base_speed + upgrade_speed) * (1.0 + relic_speed_pct / 100.0);
        self.gp_multiplier =
            (self.base_gp_multiplier + upgrade_gp_mult) * (1.0 + relic_gp_mult_pct / 100.0);
        self.xp_multiplier =
            (self.base_xp_multiplier + upgrade_xp_mult) * (1.0 + relic_xp_mult_pct / 100.0);
        self.crit_chance = (self.base_crit_chance + upgrade_crit + relic_crit).min(0.75);
        self.auto_speed = upgrade_auto;
    }
}
