use serde::{Deserialize, Serialize};

use super::item::ItemInstance;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Inventory {
    pub items: Vec<ItemInstance>,
}

impl Inventory {
    pub fn add(&mut self, mut item: ItemInstance) {
        // Try to find existing item with same id and rarity
        if let Some(existing) = self.items.iter_mut().find(|i| i.id == item.id && i.rarity == item.rarity) {
            // Combine: increment count, average the values
            let old_total_gp = existing.gp_value * existing.count as u64;
            let old_total_xp = existing.xp_value * existing.count as u64;
            let new_count = existing.count + 1;

            existing.gp_value = (old_total_gp + item.gp_value) / new_count as u64;
            existing.xp_value = (old_total_xp + item.xp_value) / new_count as u64;
            existing.count = new_count;
            existing.is_crit = existing.is_crit || item.is_crit; // Keep crit flag if any were crit
        } else {
            // New item
            item.count = 1;
            self.items.push(item);
        }
    }

    pub fn count(&self) -> usize {
        self.items.iter().map(|i| i.count as usize).sum()
    }
}
