use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::data::skills::{all_skills, get_skill};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SkillTreeState {
    pub learned: HashSet<String>,
    pub skill_points: u32,
}

impl SkillTreeState {
    pub fn has_skill(&self, id: &str) -> bool {
        self.learned.contains(id)
    }

    pub fn can_learn(&self, id: &str) -> bool {
        if self.has_skill(id) {
            return false;
        }
        if self.skill_points == 0 {
            return false;
        }
        if let Some(skill) = get_skill(id) {
            skill
                .prerequisites
                .iter()
                .all(|pre| self.has_skill(pre))
        } else {
            false
        }
    }

    pub fn learn(&mut self, id: &str) -> bool {
        if !self.can_learn(id) {
            return false;
        }
        self.learned.insert(id.to_string());
        self.skill_points -= 1;
        true
    }

    pub fn total_learned(&self) -> u32 {
        self.learned.len() as u32
    }

    /// Migrate from old save: grant retroactive skill points based on level,
    /// and auto-learn skills for old key/auto_opener upgrades.
    pub fn migrate_from_old_save(
        &mut self,
        player_level: u32,
        old_iron_key: bool,
        old_silver_key: bool,
        old_gold_key: bool,
        old_auto_opener: bool,
    ) {
        // Auto-convert old key upgrades to skills
        if old_iron_key && !self.has_skill("iron_key") {
            self.learned.insert("iron_key".to_string());
        }
        if old_silver_key && !self.has_skill("silver_key") {
            // Must have iron_key as prerequisite
            self.learned.insert("iron_key".to_string());
            self.learned.insert("silver_key".to_string());
        }
        if old_gold_key && !self.has_skill("gold_key") {
            self.learned.insert("iron_key".to_string());
            self.learned.insert("silver_key".to_string());
            self.learned.insert("gold_key".to_string());
        }
        if old_auto_opener && !self.has_skill("auto_opener") {
            // Auto opener requires swift_hands
            self.learned.insert("swift_hands".to_string());
            self.learned.insert("auto_opener".to_string());
        }

        // Grant retroactive skill points: level - 1 total points (level 1 = 0 points)
        // minus already learned skills
        let total_points_earned = player_level.saturating_sub(1);
        self.skill_points = total_points_earned.saturating_sub(self.total_learned());
    }

    /// Returns a flat list of all skill IDs in display order (grouped by branch).
    pub fn display_order() -> Vec<&'static str> {
        all_skills().iter().map(|s| s.id).collect()
    }
}
