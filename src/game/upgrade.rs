use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpgradeState {
    pub levels: HashMap<String, u32>,
}

impl UpgradeState {
    pub fn get_level(&self, id: &str) -> u32 {
        self.levels.get(id).copied().unwrap_or(0)
    }

    pub fn increment(&mut self, id: &str) {
        *self.levels.entry(id.to_string()).or_insert(0) += 1;
    }
}
