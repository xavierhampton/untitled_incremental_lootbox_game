use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::data::rebirth_skills::{all_rebirth_skills, get_rebirth_skill};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RebirthState {
    pub rebirth_count: u32,
    pub total_essence_earned: u64,
    pub essence: u64,
    pub rebirth_skills: HashSet<String>,
    pub gp_earned_this_run: u64,
    pub highest_level_ever: u32,
}

impl RebirthState {
    pub fn min_level_for_rebirth(&self) -> u32 {
        25 + self.rebirth_count * 5
    }

    pub fn can_rebirth(&self, current_level: u32) -> bool {
        current_level >= self.min_level_for_rebirth()
    }

    pub fn calculate_essence_reward(&self, level: u32, gp: u64) -> u64 {
        let base = (level as f64).powf(1.8) + (gp as f64).log10().max(0.0) * 10.0;
        let scaling = 1.0 + self.rebirth_count as f64 * 0.1;
        let mut essence = (base * scaling) as u64;

        // Essence Siphon rebirth skill: +20% essence
        if self.has_rebirth_skill("rb_essence_boost") {
            essence = (essence as f64 * 1.2) as u64;
        }
        // Ascension rebirth skill: +50% essence
        if self.has_rebirth_skill("rb_ascension") {
            essence = (essence as f64 * 1.5) as u64;
        }

        essence.max(1)
    }

    pub fn has_rebirth_skill(&self, id: &str) -> bool {
        self.rebirth_skills.contains(id)
    }

    pub fn can_learn_rebirth_skill(&self, id: &str) -> bool {
        if self.has_rebirth_skill(id) {
            return false;
        }
        if let Some(skill) = get_rebirth_skill(id) {
            if self.essence < skill.essence_cost {
                return false;
            }
            skill
                .prerequisites
                .iter()
                .all(|pre| self.has_rebirth_skill(pre))
        } else {
            false
        }
    }

    pub fn learn_rebirth_skill(&mut self, id: &str) -> bool {
        if !self.can_learn_rebirth_skill(id) {
            return false;
        }
        if let Some(skill) = get_rebirth_skill(id) {
            self.essence -= skill.essence_cost;
            self.rebirth_skills.insert(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn rebirth_skill_display_order() -> Vec<&'static str> {
        all_rebirth_skills().iter().map(|s| s.id).collect()
    }
}
