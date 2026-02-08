use crate::game::item::Rarity;

#[derive(Debug, Clone)]
pub enum RelicEffect {
    FlatLuck(f64),
    PercentSpeed(f64),
    PercentGpMult(f64),
    PercentXpMult(f64),
    FlatCrit(f64),
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

pub fn all_relics() -> Vec<RelicDef> {
    vec![
        // Rare relics (3)
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

        // Epic relics (3)
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

        // Legendary relics (2)
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
    ]
}

pub fn get_relic(id: &str) -> Option<&'static RelicDef> {
    use std::sync::LazyLock;
    static RELICS: LazyLock<Vec<RelicDef>> = LazyLock::new(all_relics);
    RELICS.iter().find(|r| r.id == id)
}

pub fn relic_stat_totals(equipped: &[String]) -> (f64, f64, f64, f64, f64) {
    let mut luck = 0.0;
    let mut speed_pct = 0.0;
    let mut gp_pct = 0.0;
    let mut xp_pct = 0.0;
    let mut crit = 0.0;

    for id in equipped {
        if let Some(relic) = get_relic(id) {
            accumulate_effect(&relic.effect, &mut luck, &mut speed_pct, &mut gp_pct, &mut xp_pct, &mut crit);
        }
    }

    (luck, speed_pct, gp_pct, xp_pct, crit)
}

fn accumulate_effect(effect: &RelicEffect, luck: &mut f64, speed: &mut f64, gp: &mut f64, xp: &mut f64, crit: &mut f64) {
    match effect {
        RelicEffect::FlatLuck(v) => *luck += v,
        RelicEffect::PercentSpeed(v) => *speed += v,
        RelicEffect::PercentGpMult(v) => *gp += v,
        RelicEffect::PercentXpMult(v) => *xp += v,
        RelicEffect::FlatCrit(v) => *crit += v,
        RelicEffect::Compound(effects) => {
            for e in effects {
                accumulate_effect(e, luck, speed, gp, xp, crit);
            }
        }
    }
}
