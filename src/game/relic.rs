use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RelicState {
    pub owned: Vec<String>,
    pub equipped: Vec<String>,
}

impl RelicState {
    pub const MAX_EQUIPPED: usize = 3;

    pub fn owns(&self, id: &str) -> bool {
        self.owned.iter().any(|r| r == id)
    }

    pub fn is_equipped(&self, id: &str) -> bool {
        self.equipped.iter().any(|r| r == id)
    }

    pub fn add_relic(&mut self, id: String) {
        if !self.owns(&id) {
            self.owned.push(id);
        }
    }

    pub fn equip(&mut self, id: &str) -> bool {
        if self.equipped.len() >= Self::MAX_EQUIPPED {
            return false;
        }
        if !self.owns(id) || self.is_equipped(id) {
            return false;
        }
        self.equipped.push(id.to_string());
        true
    }

    pub fn unequip(&mut self, id: &str) -> bool {
        if let Some(pos) = self.equipped.iter().position(|r| r == id) {
            self.equipped.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn toggle_equip(&mut self, id: &str) -> bool {
        if self.is_equipped(id) {
            self.unequip(id)
        } else {
            self.equip(id)
        }
    }
}
