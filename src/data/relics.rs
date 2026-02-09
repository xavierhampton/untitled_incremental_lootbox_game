use crate::game::item::Rarity;

#[derive(Debug, Clone)]
pub enum RelicEffect {
    FlatLuck(f64),
    PercentSpeed(f64),
    PercentGpMult(f64),
    PercentXpMult(f64),
    FlatCrit(f64),
    PercentCritMult(f64),
    FlatMultiDrop(f64),
    PercentRelicDrop(f64),
    Compound(Vec<RelicEffect>),
}

#[derive(Debug, Clone)]
pub struct RelicDef {
    pub id: &'static str,
    pub name: &'static str,
    pub rarity: Rarity,
    pub description: &'static str,
    pub effect: RelicEffect,
    pub min_chest_tier: usize, // index into ChestType::ALL
}

#[derive(Debug, Clone, Default)]
pub struct RelicStatTotals {
    pub luck: f64,
    pub speed_pct: f64,
    pub gp_pct: f64,
    pub xp_pct: f64,
    pub crit: f64,
    pub crit_mult: f64,
    pub multi_drop: f64,
    pub relic_drop_pct: f64,
}

pub fn all_relics() -> Vec<RelicDef> {
    vec![
        // === Uncommon relics (4) ===
        RelicDef {
            id: "lucky_coin",
            name: "Lucky Coin",
            rarity: Rarity::Uncommon,
            description: "+2 Luck",
            effect: RelicEffect::FlatLuck(2.0),
            min_chest_tier: 2, // Silver+
        },
        RelicDef {
            id: "wind_charm",
            name: "Wind Charm",
            rarity: Rarity::Uncommon,
            description: "+10% Speed",
            effect: RelicEffect::PercentSpeed(10.0),
            min_chest_tier: 2,
        },
        RelicDef {
            id: "bronze_idol",
            name: "Bronze Idol",
            rarity: Rarity::Uncommon,
            description: "+10% GP",
            effect: RelicEffect::PercentGpMult(10.0),
            min_chest_tier: 2,
        },
        RelicDef {
            id: "study_lens",
            name: "Study Lens",
            rarity: Rarity::Uncommon,
            description: "+10% XP",
            effect: RelicEffect::PercentXpMult(10.0),
            min_chest_tier: 2,
        },
        RelicDef {
            id: "copper_compass",
            name: "Copper Compass",
            rarity: Rarity::Uncommon,
            description: "+2% Crit",
            effect: RelicEffect::FlatCrit(0.02),
            min_chest_tier: 2,
        },
        RelicDef {
            id: "travelers_flask",
            name: "Traveler's Flask",
            rarity: Rarity::Uncommon,
            description: "+5% GP, +5% XP",
            effect: RelicEffect::Compound(vec![
                RelicEffect::PercentGpMult(5.0),
                RelicEffect::PercentXpMult(5.0),
            ]),
            min_chest_tier: 2,
        },
        RelicDef {
            id: "iron_horseshoe",
            name: "Iron Horseshoe",
            rarity: Rarity::Uncommon,
            description: "+3 Luck",
            effect: RelicEffect::FlatLuck(3.0),
            min_chest_tier: 2,
        },
        RelicDef {
            id: "tin_whistle",
            name: "Tin Whistle",
            rarity: Rarity::Uncommon,
            description: "+8% Speed",
            effect: RelicEffect::PercentSpeed(8.0),
            min_chest_tier: 2,
        },
        // === Rare relics (14) ===
        RelicDef {
            id: "rabbits_foot",
            name: "Rabbit's Foot",
            rarity: Rarity::Rare,
            description: "+3 Luck",
            effect: RelicEffect::FlatLuck(3.0),
            min_chest_tier: 3, // Gold+
        },
        RelicDef {
            id: "clockwork_gear",
            name: "Clockwork Gear",
            rarity: Rarity::Rare,
            description: "+15% Speed",
            effect: RelicEffect::PercentSpeed(15.0),
            min_chest_tier: 3,
        },
        RelicDef {
            id: "golden_idol",
            name: "Golden Idol",
            rarity: Rarity::Rare,
            description: "+20% GP",
            effect: RelicEffect::PercentGpMult(20.0),
            min_chest_tier: 3,
        },
        RelicDef {
            id: "serpent_scale",
            name: "Serpent Scale",
            rarity: Rarity::Rare,
            description: "+3% Crit, +10% GP",
            effect: RelicEffect::Compound(vec![
                RelicEffect::FlatCrit(0.03),
                RelicEffect::PercentGpMult(10.0),
            ]),
            min_chest_tier: 3,
        },
        RelicDef {
            id: "quicksilver_vial",
            name: "Quicksilver Vial",
            rarity: Rarity::Rare,
            description: "+20% Speed, +5% XP",
            effect: RelicEffect::Compound(vec![
                RelicEffect::PercentSpeed(20.0),
                RelicEffect::PercentXpMult(5.0),
            ]),
            min_chest_tier: 3,
        },
        RelicDef {
            id: "tome_of_greed",
            name: "Tome of Greed",
            rarity: Rarity::Rare,
            description: "+25% GP, -5% XP",
            effect: RelicEffect::Compound(vec![
                RelicEffect::PercentGpMult(25.0),
                RelicEffect::PercentXpMult(-5.0),
            ]),
            min_chest_tier: 3,
        },
        RelicDef {
            id: "scavengers_pouch",
            name: "Scavenger's Pouch",
            rarity: Rarity::Rare,
            description: "+5% Multi-Drop, +2 Luck",
            effect: RelicEffect::Compound(vec![
                RelicEffect::FlatMultiDrop(0.05),
                RelicEffect::FlatLuck(2.0),
            ]),
            min_chest_tier: 3,
        },
        RelicDef {
            id: "lodestone",
            name: "Lodestone",
            rarity: Rarity::Rare,
            description: "+10% Relic Drop Chance",
            effect: RelicEffect::PercentRelicDrop(10.0),
            min_chest_tier: 3,
        },
        RelicDef {
            id: "moonstone_ring",
            name: "Moonstone Ring",
            rarity: Rarity::Rare,
            description: "+5 Luck, +10% XP",
            effect: RelicEffect::Compound(vec![
                RelicEffect::FlatLuck(5.0),
                RelicEffect::PercentXpMult(10.0),
            ]),
            min_chest_tier: 3,
        },
        RelicDef {
            id: "frost_prism",
            name: "Frost Prism",
            rarity: Rarity::Rare,
            description: "+18% Speed, +3% Crit",
            effect: RelicEffect::Compound(vec![
                RelicEffect::PercentSpeed(18.0),
                RelicEffect::FlatCrit(0.03),
            ]),
            min_chest_tier: 3,
        },
        RelicDef {
            id: "bloodstone_amulet",
            name: "Bloodstone Amulet",
            rarity: Rarity::Rare,
            description: "+0.3x Crit Mult",
            effect: RelicEffect::PercentCritMult(0.3),
            min_chest_tier: 3,
        },
        RelicDef {
            id: "merchants_scale",
            name: "Merchant's Scale",
            rarity: Rarity::Rare,
            description: "+22% GP",
            effect: RelicEffect::PercentGpMult(22.0),
            min_chest_tier: 3,
        },
        RelicDef {
            id: "storm_feather",
            name: "Storm Feather",
            rarity: Rarity::Rare,
            description: "+25% Speed",
            effect: RelicEffect::PercentSpeed(25.0),
            min_chest_tier: 3,
        },
        RelicDef {
            id: "hunters_trophy",
            name: "Hunter's Trophy",
            rarity: Rarity::Rare,
            description: "+8% Multi-Drop, +5% GP",
            effect: RelicEffect::Compound(vec![
                RelicEffect::FlatMultiDrop(0.08),
                RelicEffect::PercentGpMult(5.0),
            ]),
            min_chest_tier: 3,
        },
        // === Epic relics (14) ===
        RelicDef {
            id: "chaos_orb",
            name: "Chaos Orb",
            rarity: Rarity::Epic,
            description: "+5 Luck, +5% Crit",
            effect: RelicEffect::Compound(vec![
                RelicEffect::FlatLuck(5.0),
                RelicEffect::FlatCrit(0.05),
            ]),
            min_chest_tier: 4, // Crystal+
        },
        RelicDef {
            id: "temporal_lens",
            name: "Temporal Lens",
            rarity: Rarity::Epic,
            description: "+25% Speed, +15% XP",
            effect: RelicEffect::Compound(vec![
                RelicEffect::PercentSpeed(25.0),
                RelicEffect::PercentXpMult(15.0),
            ]),
            min_chest_tier: 4,
        },
        RelicDef {
            id: "dragons_hoard",
            name: "Dragon's Hoard",
            rarity: Rarity::Epic,
            description: "+35% GP",
            effect: RelicEffect::PercentGpMult(35.0),
            min_chest_tier: 4,
        },
        RelicDef {
            id: "crown_of_thorns",
            name: "Crown of Thorns",
            rarity: Rarity::Epic,
            description: "+0.5x Crit Mult, +5% Crit",
            effect: RelicEffect::Compound(vec![
                RelicEffect::PercentCritMult(0.5),
                RelicEffect::FlatCrit(0.05),
            ]),
            min_chest_tier: 4,
        },
        RelicDef {
            id: "phoenix_heart",
            name: "Phoenix Heart",
            rarity: Rarity::Epic,
            description: "+30% GP, +20% XP",
            effect: RelicEffect::Compound(vec![
                RelicEffect::PercentGpMult(30.0),
                RelicEffect::PercentXpMult(20.0),
            ]),
            min_chest_tier: 4,
        },
        RelicDef {
            id: "abyssal_mirror",
            name: "Abyssal Mirror",
            rarity: Rarity::Epic,
            description: "+8 Luck, +15% GP",
            effect: RelicEffect::Compound(vec![
                RelicEffect::FlatLuck(8.0),
                RelicEffect::PercentGpMult(15.0),
            ]),
            min_chest_tier: 4,
        },
        RelicDef {
            id: "chrono_shard",
            name: "Chrono Shard",
            rarity: Rarity::Epic,
            description: "+35% Speed, +10% GP",
            effect: RelicEffect::Compound(vec![
                RelicEffect::PercentSpeed(35.0),
                RelicEffect::PercentGpMult(10.0),
            ]),
            min_chest_tier: 4,
        },
        RelicDef {
            id: "soul_lantern",
            name: "Soul Lantern",
            rarity: Rarity::Epic,
            description: "+25% XP, +10% Multi-Drop",
            effect: RelicEffect::Compound(vec![
                RelicEffect::PercentXpMult(25.0),
                RelicEffect::FlatMultiDrop(0.10),
            ]),
            min_chest_tier: 4,
        },
        RelicDef {
            id: "eclipse_medallion",
            name: "Eclipse Medallion",
            rarity: Rarity::Epic,
            description: "+10% Crit, +0.5x Crit Mult",
            effect: RelicEffect::Compound(vec![
                RelicEffect::FlatCrit(0.10),
                RelicEffect::PercentCritMult(0.5),
            ]),
            min_chest_tier: 4,
        },
        RelicDef {
            id: "warden_sigil",
            name: "Warden's Sigil",
            rarity: Rarity::Epic,
            description: "+10 Luck, +20% Speed",
            effect: RelicEffect::Compound(vec![
                RelicEffect::FlatLuck(10.0),
                RelicEffect::PercentSpeed(20.0),
            ]),
            min_chest_tier: 4,
        },
        RelicDef {
            id: "golden_chalice",
            name: "Golden Chalice",
            rarity: Rarity::Epic,
            description: "+40% GP",
            effect: RelicEffect::PercentGpMult(40.0),
            min_chest_tier: 4,
        },
        RelicDef {
            id: "nightmare_lens",
            name: "Nightmare Lens",
            rarity: Rarity::Epic,
            description: "+15% Relic Drop, +5 Luck",
            effect: RelicEffect::Compound(vec![
                RelicEffect::PercentRelicDrop(15.0),
                RelicEffect::FlatLuck(5.0),
            ]),
            min_chest_tier: 4,
        },
        RelicDef {
            id: "stormcaller_horn",
            name: "Stormcaller's Horn",
            rarity: Rarity::Epic,
            description: "+40% Speed, +10% XP",
            effect: RelicEffect::Compound(vec![
                RelicEffect::PercentSpeed(40.0),
                RelicEffect::PercentXpMult(10.0),
            ]),
            min_chest_tier: 4,
        },
        // === Legendary relics (12) ===
        RelicDef {
            id: "star_heart",
            name: "Star Heart",
            rarity: Rarity::Legendary,
            description: "+10 Luck, +10% Crit, +30% GP",
            effect: RelicEffect::Compound(vec![
                RelicEffect::FlatLuck(10.0),
                RelicEffect::FlatCrit(0.10),
                RelicEffect::PercentGpMult(30.0),
            ]),
            min_chest_tier: 5, // Shadow+
        },
        RelicDef {
            id: "void_anchor",
            name: "Void Anchor",
            rarity: Rarity::Legendary,
            description: "+40% Speed, +40% GP, +25% XP",
            effect: RelicEffect::Compound(vec![
                RelicEffect::PercentSpeed(40.0),
                RelicEffect::PercentGpMult(40.0),
                RelicEffect::PercentXpMult(25.0),
            ]),
            min_chest_tier: 5,
        },
        RelicDef {
            id: "world_tree_seed",
            name: "World Tree Seed",
            rarity: Rarity::Legendary,
            description: "+15 Luck, +30% XP, +20% GP",
            effect: RelicEffect::Compound(vec![
                RelicEffect::FlatLuck(15.0),
                RelicEffect::PercentXpMult(30.0),
                RelicEffect::PercentGpMult(20.0),
            ]),
            min_chest_tier: 5,
        },
        RelicDef {
            id: "entropy_engine",
            name: "Entropy Engine",
            rarity: Rarity::Legendary,
            description: "+50% Speed, +1.0x Crit Mult",
            effect: RelicEffect::Compound(vec![
                RelicEffect::PercentSpeed(50.0),
                RelicEffect::PercentCritMult(1.0),
            ]),
            min_chest_tier: 5,
        },
        RelicDef {
            id: "hand_of_midas",
            name: "Hand of Midas",
            rarity: Rarity::Legendary,
            description: "+60% GP, +10% Crit",
            effect: RelicEffect::Compound(vec![
                RelicEffect::PercentGpMult(60.0),
                RelicEffect::FlatCrit(0.10),
            ]),
            min_chest_tier: 5,
        },
        RelicDef {
            id: "omniscient_eye",
            name: "Omniscient Eye",
            rarity: Rarity::Legendary,
            description: "+20% Relic Drop, +15% Multi-Drop, +10 Luck",
            effect: RelicEffect::Compound(vec![
                RelicEffect::PercentRelicDrop(20.0),
                RelicEffect::FlatMultiDrop(0.15),
                RelicEffect::FlatLuck(10.0),
            ]),
            min_chest_tier: 6, // Void
        },
        RelicDef {
            id: "primordial_flame",
            name: "Primordial Flame",
            rarity: Rarity::Legendary,
            description: "+15% Crit, +1.5x Crit Mult, +25% GP",
            effect: RelicEffect::Compound(vec![
                RelicEffect::FlatCrit(0.15),
                RelicEffect::PercentCritMult(1.5),
                RelicEffect::PercentGpMult(25.0),
            ]),
            min_chest_tier: 6,
        },
        RelicDef {
            id: "eternity_loop",
            name: "Eternity Loop",
            rarity: Rarity::Legendary,
            description: "+50% GP, +50% XP, +50% Speed",
            effect: RelicEffect::Compound(vec![
                RelicEffect::PercentGpMult(50.0),
                RelicEffect::PercentXpMult(50.0),
                RelicEffect::PercentSpeed(50.0),
            ]),
            min_chest_tier: 6,
        },
        RelicDef {
            id: "astral_crown",
            name: "Astral Crown",
            rarity: Rarity::Legendary,
            description: "+20 Luck, +10% Crit, +20% GP",
            effect: RelicEffect::Compound(vec![
                RelicEffect::FlatLuck(20.0),
                RelicEffect::FlatCrit(0.10),
                RelicEffect::PercentGpMult(20.0),
            ]),
            min_chest_tier: 5,
        },
        RelicDef {
            id: "soul_forge_hammer",
            name: "Soul Forge Hammer",
            rarity: Rarity::Legendary,
            description: "+2.0x Crit Mult, +5% Crit",
            effect: RelicEffect::Compound(vec![
                RelicEffect::PercentCritMult(2.0),
                RelicEffect::FlatCrit(0.05),
            ]),
            min_chest_tier: 6,
        },
        RelicDef {
            id: "dragon_king_scale",
            name: "Dragon King's Scale",
            rarity: Rarity::Legendary,
            description: "+70% GP, +15 Luck",
            effect: RelicEffect::Compound(vec![
                RelicEffect::PercentGpMult(70.0),
                RelicEffect::FlatLuck(15.0),
            ]),
            min_chest_tier: 6,
        },
        RelicDef {
            id: "temporal_anchor",
            name: "Temporal Anchor",
            rarity: Rarity::Legendary,
            description: "+60% Speed, +30% XP",
            effect: RelicEffect::Compound(vec![
                RelicEffect::PercentSpeed(60.0),
                RelicEffect::PercentXpMult(30.0),
            ]),
            min_chest_tier: 5,
        },
        // === Mythic relics (6) ===
        RelicDef {
            id: "genesis_spark",
            name: "Genesis Spark",
            rarity: Rarity::Mythic,
            description: "+30 Luck, +20% Crit, +100% GP",
            effect: RelicEffect::Compound(vec![
                RelicEffect::FlatLuck(30.0),
                RelicEffect::FlatCrit(0.20),
                RelicEffect::PercentGpMult(100.0),
            ]),
            min_chest_tier: 6,
        },
        RelicDef {
            id: "void_throne_shard",
            name: "Void Throne Shard",
            rarity: Rarity::Mythic,
            description: "+100% Speed, +50% GP, +50% XP",
            effect: RelicEffect::Compound(vec![
                RelicEffect::PercentSpeed(100.0),
                RelicEffect::PercentGpMult(50.0),
                RelicEffect::PercentXpMult(50.0),
            ]),
            min_chest_tier: 6,
        },
        RelicDef {
            id: "omega_prism",
            name: "Omega Prism",
            rarity: Rarity::Mythic,
            description: "+3.0x Crit Mult, +15% Crit, +30% GP",
            effect: RelicEffect::Compound(vec![
                RelicEffect::PercentCritMult(3.0),
                RelicEffect::FlatCrit(0.15),
                RelicEffect::PercentGpMult(30.0),
            ]),
            min_chest_tier: 6,
        },
        RelicDef {
            id: "fate_weaver_loom",
            name: "Fate Weaver's Loom",
            rarity: Rarity::Mythic,
            description: "+40% Relic Drop, +25% Multi-Drop, +20 Luck",
            effect: RelicEffect::Compound(vec![
                RelicEffect::PercentRelicDrop(40.0),
                RelicEffect::FlatMultiDrop(0.25),
                RelicEffect::FlatLuck(20.0),
            ]),
            min_chest_tier: 6,
        },
        RelicDef {
            id: "celestial_engine",
            name: "Celestial Engine",
            rarity: Rarity::Mythic,
            description: "+80% GP, +80% XP, +80% Speed",
            effect: RelicEffect::Compound(vec![
                RelicEffect::PercentGpMult(80.0),
                RelicEffect::PercentXpMult(80.0),
                RelicEffect::PercentSpeed(80.0),
            ]),
            min_chest_tier: 6,
        },
        RelicDef {
            id: "singularity_core",
            name: "Singularity Core",
            rarity: Rarity::Mythic,
            description: "+50 Luck, +25% Crit, +4.0x Crit Mult",
            effect: RelicEffect::Compound(vec![
                RelicEffect::FlatLuck(50.0),
                RelicEffect::FlatCrit(0.25),
                RelicEffect::PercentCritMult(4.0),
            ]),
            min_chest_tier: 6,
        },
    ]
}

pub fn get_relic(id: &str) -> Option<&'static RelicDef> {
    use std::sync::LazyLock;
    static RELICS: LazyLock<Vec<RelicDef>> = LazyLock::new(all_relics);
    RELICS.iter().find(|r| r.id == id)
}

pub fn relic_stat_totals(equipped: &[String]) -> RelicStatTotals {
    let mut totals = RelicStatTotals::default();

    for id in equipped {
        if let Some(relic) = get_relic(id) {
            accumulate_effect(&relic.effect, &mut totals);
        }
    }

    totals
}

fn accumulate_effect(effect: &RelicEffect, totals: &mut RelicStatTotals) {
    match effect {
        RelicEffect::FlatLuck(v) => totals.luck += v,
        RelicEffect::PercentSpeed(v) => totals.speed_pct += v,
        RelicEffect::PercentGpMult(v) => totals.gp_pct += v,
        RelicEffect::PercentXpMult(v) => totals.xp_pct += v,
        RelicEffect::FlatCrit(v) => totals.crit += v,
        RelicEffect::PercentCritMult(v) => totals.crit_mult += v,
        RelicEffect::FlatMultiDrop(v) => totals.multi_drop += v,
        RelicEffect::PercentRelicDrop(v) => totals.relic_drop_pct += v,
        RelicEffect::Compound(effects) => {
            for e in effects {
                accumulate_effect(e, totals);
            }
        }
    }
}
