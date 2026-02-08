use crate::game::chest::ChestType;
use crate::game::item::ItemId;

pub struct LootEntry {
    pub item_id: ItemId,
    pub weight: f64,
}

pub struct LootTable {
    pub entries: Vec<LootEntry>,
}

impl LootTable {
    pub fn weighted_entries(&self, luck: f64) -> Vec<(usize, f64)> {
        // Luck boosts rarer items: items later in the table (rarer) get luck bonus
        self.entries
            .iter()
            .enumerate()
            .map(|(i, entry)| {
                let rarity_bonus = 1.0 + (i as f64 * luck * 0.02);
                (i, entry.weight * rarity_bonus)
            })
            .collect()
    }
}

pub fn loot_table_for(chest: ChestType) -> LootTable {
    match chest {
        ChestType::Wooden => LootTable {
            entries: vec![
                LootEntry { item_id: "rusty_coin", weight: 20.0 },
                LootEntry { item_id: "wooden_button", weight: 18.0 },
                LootEntry { item_id: "pebble", weight: 18.0 },
                LootEntry { item_id: "torn_cloth", weight: 15.0 },
                LootEntry { item_id: "bent_nail", weight: 15.0 },
                LootEntry { item_id: "clay_bead", weight: 12.0 },
                LootEntry { item_id: "bone_fragment", weight: 10.0 },
                LootEntry { item_id: "glass_shard", weight: 8.0 },
                LootEntry { item_id: "silver_coin", weight: 3.0 },
                LootEntry { item_id: "jade_pendant", weight: 1.0 },
            ],
        },
        ChestType::Iron => LootTable {
            entries: vec![
                LootEntry { item_id: "rusty_coin", weight: 12.0 },
                LootEntry { item_id: "iron_ring", weight: 15.0 },
                LootEntry { item_id: "clay_bead", weight: 12.0 },
                LootEntry { item_id: "bone_fragment", weight: 10.0 },
                LootEntry { item_id: "feather", weight: 10.0 },
                LootEntry { item_id: "silver_coin", weight: 10.0 },
                LootEntry { item_id: "jade_pendant", weight: 8.0 },
                LootEntry { item_id: "quartz_crystal", weight: 7.0 },
                LootEntry { item_id: "bronze_dagger", weight: 6.0 },
                LootEntry { item_id: "gold_bar", weight: 2.0 },
            ],
        },
        ChestType::Silver => LootTable {
            entries: vec![
                LootEntry { item_id: "iron_ring", weight: 10.0 },
                LootEntry { item_id: "glass_shard", weight: 8.0 },
                LootEntry { item_id: "silver_coin", weight: 14.0 },
                LootEntry { item_id: "jade_pendant", weight: 12.0 },
                LootEntry { item_id: "quartz_crystal", weight: 12.0 },
                LootEntry { item_id: "silk_ribbon", weight: 10.0 },
                LootEntry { item_id: "amber_chunk", weight: 10.0 },
                LootEntry { item_id: "gold_bar", weight: 6.0 },
                LootEntry { item_id: "enchanted_scroll", weight: 5.0 },
                LootEntry { item_id: "sapphire_ring", weight: 3.0 },
            ],
        },
        ChestType::Gold => LootTable {
            entries: vec![
                LootEntry { item_id: "silver_coin", weight: 8.0 },
                LootEntry { item_id: "amber_chunk", weight: 10.0 },
                LootEntry { item_id: "carved_rune", weight: 10.0 },
                LootEntry { item_id: "moon_pearl", weight: 10.0 },
                LootEntry { item_id: "gold_bar", weight: 12.0 },
                LootEntry { item_id: "enchanted_scroll", weight: 10.0 },
                LootEntry { item_id: "sapphire_ring", weight: 8.0 },
                LootEntry { item_id: "mithril_shard", weight: 7.0 },
                LootEntry { item_id: "void_crystal", weight: 3.0 },
                LootEntry { item_id: "demon_heart", weight: 2.0 },
            ],
        },
        ChestType::Crystal => LootTable {
            entries: vec![
                LootEntry { item_id: "moon_pearl", weight: 8.0 },
                LootEntry { item_id: "carved_rune", weight: 8.0 },
                LootEntry { item_id: "gold_bar", weight: 10.0 },
                LootEntry { item_id: "enchanted_scroll", weight: 10.0 },
                LootEntry { item_id: "sapphire_ring", weight: 10.0 },
                LootEntry { item_id: "phoenix_feather", weight: 10.0 },
                LootEntry { item_id: "dragon_scale", weight: 10.0 },
                LootEntry { item_id: "void_crystal", weight: 8.0 },
                LootEntry { item_id: "astral_compass", weight: 5.0 },
                LootEntry { item_id: "crown_of_ages", weight: 1.0 },
            ],
        },
        ChestType::Shadow => LootTable {
            entries: vec![
                LootEntry { item_id: "mithril_shard", weight: 8.0 },
                LootEntry { item_id: "phoenix_feather", weight: 10.0 },
                LootEntry { item_id: "dragon_scale", weight: 12.0 },
                LootEntry { item_id: "void_crystal", weight: 12.0 },
                LootEntry { item_id: "demon_heart", weight: 12.0 },
                LootEntry { item_id: "astral_compass", weight: 10.0 },
                LootEntry { item_id: "titan_bone", weight: 10.0 },
                LootEntry { item_id: "crown_of_ages", weight: 4.0 },
                LootEntry { item_id: "infinity_gem", weight: 3.0 },
                LootEntry { item_id: "godslayer_blade", weight: 1.5 },
            ],
        },
        ChestType::Void => LootTable {
            entries: vec![
                LootEntry { item_id: "dragon_scale", weight: 8.0 },
                LootEntry { item_id: "void_crystal", weight: 12.0 },
                LootEntry { item_id: "demon_heart", weight: 12.0 },
                LootEntry { item_id: "astral_compass", weight: 12.0 },
                LootEntry { item_id: "titan_bone", weight: 12.0 },
                LootEntry { item_id: "crown_of_ages", weight: 8.0 },
                LootEntry { item_id: "infinity_gem", weight: 7.0 },
                LootEntry { item_id: "godslayer_blade", weight: 5.0 },
            ],
        },
    }
}
