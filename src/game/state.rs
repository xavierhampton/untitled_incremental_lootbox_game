use serde::{Deserialize, Serialize};

use super::chest::{ChestProgress, ChestType};
use super::inventory::Inventory;
use super::player::Player;
use super::relic::RelicState;
use super::skill_tree::SkillTreeState;
use super::upgrade::UpgradeState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifetimeStats {
    pub chests_opened: u64,
    pub items_found: u64,
    pub total_gp_earned: u64,
    pub total_xp_earned: u64,
    pub legendaries_found: u64,
    pub epics_found: u64,
    pub rares_found: u64,
    pub crits_rolled: u64,
    pub highest_single_gp: u64,
}

impl Default for LifetimeStats {
    fn default() -> Self {
        Self {
            chests_opened: 0,
            items_found: 0,
            total_gp_earned: 0,
            total_xp_earned: 0,
            legendaries_found: 0,
            epics_found: 0,
            rares_found: 0,
            crits_rolled: 0,
            highest_single_gp: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub player: Player,
    pub inventory: Inventory,
    pub upgrades: UpgradeState,
    pub relics: RelicState,
    #[serde(default)]
    pub skill_tree: SkillTreeState,
    pub chest_progress: ChestProgress,
    pub current_chest_type: ChestType,
    pub stats: LifetimeStats,
    pub unlocked_chests: Vec<ChestType>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            player: Player::default(),
            inventory: Inventory::default(),
            upgrades: UpgradeState::default(),
            relics: RelicState::default(),
            skill_tree: SkillTreeState::default(),
            chest_progress: ChestProgress::default(),
            current_chest_type: ChestType::Wooden,
            stats: LifetimeStats::default(),
            unlocked_chests: vec![ChestType::Wooden],
        }
    }
}
