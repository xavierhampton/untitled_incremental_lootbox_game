use serde::{Deserialize, Serialize};

use super::item::ItemInstance;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Inventory {
    pub items: Vec<ItemInstance>,
}

impl Inventory {
    pub fn add(&mut self, item: ItemInstance) {
        self.items.push(item);
    }

    pub fn count(&self) -> usize {
        self.items.len()
    }
}
