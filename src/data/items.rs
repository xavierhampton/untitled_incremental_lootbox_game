use crate::game::item::{ItemDef, Rarity};

pub fn all_items() -> Vec<ItemDef> {
    vec![
        // === Common (10) ===
        ItemDef { id: "rusty_coin", name: "Rusty Coin", rarity: Rarity::Common, base_gp: 10, base_xp: 5, description: "A tarnished copper coin." },
        ItemDef { id: "wooden_button", name: "Wooden Button", rarity: Rarity::Common, base_gp: 8, base_xp: 4, description: "Carved from oak." },
        ItemDef { id: "pebble", name: "Smooth Pebble", rarity: Rarity::Common, base_gp: 6, base_xp: 3, description: "Worn smooth by a river." },
        ItemDef { id: "torn_cloth", name: "Torn Cloth", rarity: Rarity::Common, base_gp: 7, base_xp: 4, description: "A scrap of old fabric." },
        ItemDef { id: "bent_nail", name: "Bent Nail", rarity: Rarity::Common, base_gp: 5, base_xp: 3, description: "Rusted and crooked." },
        ItemDef { id: "clay_bead", name: "Clay Bead", rarity: Rarity::Common, base_gp: 9, base_xp: 5, description: "A simple painted bead." },
        ItemDef { id: "bone_fragment", name: "Bone Fragment", rarity: Rarity::Common, base_gp: 8, base_xp: 4, description: "Origin unknown." },
        ItemDef { id: "glass_shard", name: "Glass Shard", rarity: Rarity::Common, base_gp: 7, base_xp: 3, description: "Catches the light." },
        ItemDef { id: "feather", name: "Crow Feather", rarity: Rarity::Common, base_gp: 6, base_xp: 3, description: "Ink-black plumage." },
        ItemDef { id: "iron_ring", name: "Iron Ring", rarity: Rarity::Common, base_gp: 12, base_xp: 6, description: "Plain but sturdy." },

        // === Uncommon (8) ===
        ItemDef { id: "silver_coin", name: "Silver Coin", rarity: Rarity::Uncommon, base_gp: 25, base_xp: 12, description: "Freshly minted." },
        ItemDef { id: "jade_pendant", name: "Jade Pendant", rarity: Rarity::Uncommon, base_gp: 30, base_xp: 15, description: "Cool to the touch." },
        ItemDef { id: "quartz_crystal", name: "Quartz Crystal", rarity: Rarity::Uncommon, base_gp: 28, base_xp: 14, description: "Hums faintly." },
        ItemDef { id: "bronze_dagger", name: "Bronze Dagger", rarity: Rarity::Uncommon, base_gp: 35, base_xp: 18, description: "Still sharp." },
        ItemDef { id: "silk_ribbon", name: "Silk Ribbon", rarity: Rarity::Uncommon, base_gp: 22, base_xp: 11, description: "Crimson and smooth." },
        ItemDef { id: "amber_chunk", name: "Amber Chunk", rarity: Rarity::Uncommon, base_gp: 32, base_xp: 16, description: "Contains a tiny insect." },
        ItemDef { id: "carved_rune", name: "Carved Rune", rarity: Rarity::Uncommon, base_gp: 27, base_xp: 14, description: "Ancient symbols glow." },
        ItemDef { id: "moon_pearl", name: "Moon Pearl", rarity: Rarity::Uncommon, base_gp: 33, base_xp: 17, description: "Luminescent in darkness." },

        // === Rare (6) ===
        ItemDef { id: "gold_bar", name: "Gold Bar", rarity: Rarity::Rare, base_gp: 80, base_xp: 40, description: "Heavy and pure." },
        ItemDef { id: "enchanted_scroll", name: "Enchanted Scroll", rarity: Rarity::Rare, base_gp: 90, base_xp: 45, description: "Words shift as you read." },
        ItemDef { id: "sapphire_ring", name: "Sapphire Ring", rarity: Rarity::Rare, base_gp: 100, base_xp: 50, description: "Deep ocean blue." },
        ItemDef { id: "mithril_shard", name: "Mithril Shard", rarity: Rarity::Rare, base_gp: 85, base_xp: 43, description: "Lighter than air." },
        ItemDef { id: "phoenix_feather", name: "Phoenix Feather", rarity: Rarity::Rare, base_gp: 95, base_xp: 48, description: "Warm to the touch." },
        ItemDef { id: "dragon_scale", name: "Dragon Scale", rarity: Rarity::Rare, base_gp: 110, base_xp: 55, description: "Iridescent and tough." },

        // === Epic (4) ===
        ItemDef { id: "void_crystal", name: "Void Crystal", rarity: Rarity::Epic, base_gp: 250, base_xp: 125, description: "Absorbs nearby light." },
        ItemDef { id: "demon_heart", name: "Demon Heart", rarity: Rarity::Epic, base_gp: 300, base_xp: 150, description: "Still beating." },
        ItemDef { id: "astral_compass", name: "Astral Compass", rarity: Rarity::Epic, base_gp: 275, base_xp: 138, description: "Points to other realms." },
        ItemDef { id: "titan_bone", name: "Titan Bone", rarity: Rarity::Epic, base_gp: 280, base_xp: 140, description: "From an ancient giant." },

        // === Legendary (3) ===
        ItemDef { id: "crown_of_ages", name: "Crown of Ages", rarity: Rarity::Legendary, base_gp: 1000, base_xp: 500, description: "Worn by forgotten kings." },
        ItemDef { id: "infinity_gem", name: "Infinity Gem", rarity: Rarity::Legendary, base_gp: 1200, base_xp: 600, description: "Contains a universe." },
        ItemDef { id: "godslayer_blade", name: "Godslayer Blade", rarity: Rarity::Legendary, base_gp: 1500, base_xp: 750, description: "The edge between worlds." },

        // === Mythic (3) ===
        ItemDef { id: "tear_of_creation", name: "Tear of Creation", rarity: Rarity::Mythic, base_gp: 5000, base_xp: 2500, description: "A droplet from the birth of reality." },
        ItemDef { id: "aeons_heart", name: "Aeon's Heart", rarity: Rarity::Mythic, base_gp: 6000, base_xp: 3000, description: "Pulses with the rhythm of time itself." },
        ItemDef { id: "void_sovereign_crown", name: "Void Sovereign's Crown", rarity: Rarity::Mythic, base_gp: 7500, base_xp: 3750, description: "Dominion over nothingness." },
    ]
}

pub fn get_item(id: &str) -> Option<&'static ItemDef> {
    use std::sync::LazyLock;
    static ITEMS: LazyLock<Vec<ItemDef>> = LazyLock::new(all_items);
    ITEMS.iter().find(|i| i.id == id)
}
