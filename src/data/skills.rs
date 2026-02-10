use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SkillBranch {
    Fortune,
    Speed,
    Mastery,
    Discovery,
    Alchemy,
    Chaos,
}

impl SkillBranch {
    pub const ALL: [SkillBranch; 6] = [
        SkillBranch::Fortune,
        SkillBranch::Speed,
        SkillBranch::Mastery,
        SkillBranch::Discovery,
        SkillBranch::Alchemy,
        SkillBranch::Chaos,
    ];

    pub fn label(self) -> &'static str {
        match self {
            SkillBranch::Fortune => "Fortune",
            SkillBranch::Speed => "Speed",
            SkillBranch::Mastery => "Mastery",
            SkillBranch::Discovery => "Discovery",
            SkillBranch::Alchemy => "Alchemy",
            SkillBranch::Chaos => "Chaos",
        }
    }

    pub fn color(self) -> ratatui::style::Color {
        match self {
            SkillBranch::Fortune => ratatui::style::Color::Yellow,
            SkillBranch::Speed => ratatui::style::Color::Cyan,
            SkillBranch::Mastery => ratatui::style::Color::Red,
            SkillBranch::Discovery => ratatui::style::Color::Green,
            SkillBranch::Alchemy => ratatui::style::Color::Rgb(255, 165, 0),
            SkillBranch::Chaos => ratatui::style::Color::Rgb(200, 50, 50),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SkillDef {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub branch: SkillBranch,
    pub prerequisites: &'static [&'static str],
    pub cost: u32, // Skill points required
}

pub fn all_skills() -> &'static [SkillDef] {
    use std::sync::LazyLock;
    static SKILLS: LazyLock<Vec<SkillDef>> = LazyLock::new(|| {
        vec![
            // === Fortune Branch (10) ===
            // Left path: Luck & Procs
            SkillDef {
                id: "lucky_charm",
                name: "Lucky Charm",
                description: "+5 luck, +10% GP",
                branch: SkillBranch::Fortune,
                prerequisites: &[],
                cost: 1,
            },
            SkillDef {
                id: "golden_touch",
                name: "Golden Touch",
                description: "25% chance for double GP from a chest",
                branch: SkillBranch::Fortune,
                prerequisites: &["lucky_charm"],
                cost: 1,
            },
            SkillDef {
                id: "windfall",
                name: "Windfall",
                description: "3% chance for 10x GP on any chest",
                branch: SkillBranch::Fortune,
                prerequisites: &["golden_touch"],
                cost: 2,
            },
            SkillDef {
                id: "fortune_favors",
                name: "Fortune Favors",
                description: "+15 luck, +25% GP",
                branch: SkillBranch::Fortune,
                prerequisites: &["windfall"],
                cost: 3,
            },
            SkillDef {
                id: "lucky_streak",
                name: "Lucky Streak",
                description: "Consecutive rare+ finds give +20% GP each",
                branch: SkillBranch::Fortune,
                prerequisites: &["fortune_favors"],
                cost: 3,
            },
            // Right path: Raw GP Power
            SkillDef {
                id: "treasure_sense",
                name: "Treasure Sense",
                description: "+50% GP from all sources",
                branch: SkillBranch::Fortune,
                prerequisites: &["lucky_charm"],
                cost: 1,
            },
            SkillDef {
                id: "double_or_nothing",
                name: "Double or Nothing",
                description: "30% double loot, 5% nothing",
                branch: SkillBranch::Fortune,
                prerequisites: &["treasure_sense"],
                cost: 2,
            },
            SkillDef {
                id: "high_roller",
                name: "High Roller",
                description: "+15% GP per chest tier above Wooden",
                branch: SkillBranch::Fortune,
                prerequisites: &["double_or_nothing"],
                cost: 2,
            },
            SkillDef {
                id: "gold_rush",
                name: "Gold Rush",
                description: "10% chance for bonus GP equal to item value",
                branch: SkillBranch::Fortune,
                prerequisites: &["high_roller"],
                cost: 3,
            },
            // Capstone
            SkillDef {
                id: "golden_rain",
                name: "Golden Rain",
                description: "Capstone: +80% GP, Windfall chance doubled to 6%",
                branch: SkillBranch::Fortune,
                prerequisites: &["lucky_streak", "gold_rush"],
                cost: 5,
            },
            // === Speed Branch (10) ===
            SkillDef {
                id: "swift_hands",
                name: "Swift Hands",
                description: "+30% chest speed",
                branch: SkillBranch::Speed,
                prerequisites: &[],
                cost: 1,
            },
            SkillDef {
                id: "nimble_fingers",
                name: "Nimble Fingers",
                description: "+25% chest speed (stacks)",
                branch: SkillBranch::Speed,
                prerequisites: &["swift_hands"],
                cost: 1,
            },
            SkillDef {
                id: "burst_open",
                name: "Burst Open",
                description: "15% chance to instantly open a chest",
                branch: SkillBranch::Speed,
                prerequisites: &["nimble_fingers"],
                cost: 2,
            },
            SkillDef {
                id: "quick_collect",
                name: "Quick Collect",
                description: "Auto-collect loot after 0.8 seconds",
                branch: SkillBranch::Speed,
                prerequisites: &["burst_open"],
                cost: 2,
            },
            SkillDef {
                id: "auto_opener",
                name: "Auto Opener",
                description: "Chests open and collect automatically",
                branch: SkillBranch::Speed,
                prerequisites: &["swift_hands"],
                cost: 1,
            },
            SkillDef {
                id: "idle_income",
                name: "Idle Income",
                description: "Earn GP/sec while idle (scales with level & GP mult)",
                branch: SkillBranch::Speed,
                prerequisites: &["auto_opener"],
                cost: 2,
            },
            SkillDef {
                id: "perpetual_motion",
                name: "Perpetual Motion",
                description: "Auto-collect delay halved (1s instead of 2s)",
                branch: SkillBranch::Speed,
                prerequisites: &["idle_income"],
                cost: 2,
            },
            // Speed extended
            SkillDef {
                id: "time_warp",
                name: "Time Warp",
                description: "+50% chest speed",
                branch: SkillBranch::Speed,
                prerequisites: &["quick_collect"],
                cost: 3,
            },
            SkillDef {
                id: "momentum",
                name: "Momentum",
                description: "Each consecutive chest opens 3% faster (max 60%)",
                branch: SkillBranch::Speed,
                prerequisites: &["time_warp"],
                cost: 3,
            },
            SkillDef {
                id: "temporal_mastery",
                name: "Temporal Mastery",
                description: "Capstone: +75% speed, idle income x4",
                branch: SkillBranch::Speed,
                prerequisites: &["momentum", "perpetual_motion"],
                cost: 5,
            },
            // === Mastery Branch (10) ===
            SkillDef {
                id: "critical_eye",
                name: "Critical Eye",
                description: "+5% crit chance",
                branch: SkillBranch::Mastery,
                prerequisites: &[],
                cost: 1,
            },
            SkillDef {
                id: "crit_cascade",
                name: "Crit Cascade",
                description: "Crits have 25% chance to trigger another",
                branch: SkillBranch::Mastery,
                prerequisites: &["critical_eye"],
                cost: 1,
            },
            SkillDef {
                id: "multi_drop",
                name: "Multi-Drop",
                description: "10% chance to find 2 items from one chest",
                branch: SkillBranch::Mastery,
                prerequisites: &["crit_cascade"],
                cost: 2,
            },
            SkillDef {
                id: "legendary_aura",
                name: "Legendary Aura",
                description: "Legendary items give 3x XP",
                branch: SkillBranch::Mastery,
                prerequisites: &["multi_drop"],
                cost: 2,
            },
            SkillDef {
                id: "overcharge",
                name: "Overcharge",
                description: "Crit multiplier: 2x \u{2192} 3x",
                branch: SkillBranch::Mastery,
                prerequisites: &["critical_eye"],
                cost: 1,
            },
            SkillDef {
                id: "xp_surge",
                name: "XP Surge",
                description: "Every 5th chest gives 5x XP",
                branch: SkillBranch::Mastery,
                prerequisites: &["overcharge"],
                cost: 2,
            },
            SkillDef {
                id: "midas_touch",
                name: "Midas Touch",
                description: "+100% base GP on items",
                branch: SkillBranch::Mastery,
                prerequisites: &["xp_surge"],
                cost: 2,
            },
            // Mastery extended
            SkillDef {
                id: "precision_strike",
                name: "Precision Strike",
                description: "+10% crit chance, crits give +50% XP",
                branch: SkillBranch::Mastery,
                prerequisites: &["legendary_aura"],
                cost: 3,
            },
            SkillDef {
                id: "deep_knowledge",
                name: "Deep Knowledge",
                description: "+100% XP from all sources",
                branch: SkillBranch::Mastery,
                prerequisites: &["precision_strike"],
                cost: 3,
            },
            SkillDef {
                id: "grand_mastery",
                name: "Grand Mastery",
                description: "Capstone: crit mult +2x, +25% all multipliers",
                branch: SkillBranch::Mastery,
                prerequisites: &["deep_knowledge", "midas_touch"],
                cost: 5,
            },
            // === Discovery Branch (6) ===
            SkillDef {
                id: "scavenger",
                name: "Scavenger",
                description: "5% chance for a bonus Common item",
                branch: SkillBranch::Discovery,
                prerequisites: &[],
                cost: 1,
            },
            SkillDef {
                id: "relic_hunter",
                name: "Relic Hunter",
                description: "Double relic drop chance",
                branch: SkillBranch::Discovery,
                prerequisites: &["scavenger"],
                cost: 2,
            },
            SkillDef {
                id: "recycler",
                name: "Recycler",
                description: "Auto-sell Common items for GP",
                branch: SkillBranch::Discovery,
                prerequisites: &["relic_hunter"],
                cost: 2,
            },
            // Discovery extended
            SkillDef {
                id: "cartographer",
                name: "Cartographer",
                description: "Unlock chests 3 levels earlier",
                branch: SkillBranch::Discovery,
                prerequisites: &["recycler"],
                cost: 3,
            },
            SkillDef {
                id: "deep_salvage",
                name: "Deep Salvage",
                description: "Recycled items give 3x GP, +10% relic drop",
                branch: SkillBranch::Discovery,
                prerequisites: &["cartographer"],
                cost: 3,
            },
            SkillDef {
                id: "world_explorer",
                name: "World Explorer",
                description: "Capstone: +50% all drop rates, +1 relic slot",
                branch: SkillBranch::Discovery,
                prerequisites: &["deep_salvage", "recycler"],
                cost: 5,
            },
            // === Alchemy Branch (8) ===
            SkillDef {
                id: "transmute_basics",
                name: "Transmute Basics",
                description: "Sell items from inventory for 50% GP value",
                branch: SkillBranch::Alchemy,
                prerequisites: &[],
                cost: 1,
            },
            SkillDef {
                id: "gold_synthesis",
                name: "Gold Synthesis",
                description: "Sell value increased to 75%",
                branch: SkillBranch::Alchemy,
                prerequisites: &["transmute_basics"],
                cost: 1,
            },
            SkillDef {
                id: "essence_distill",
                name: "Essence Distill",
                description: "Selling items grants +10% XP of GP value",
                branch: SkillBranch::Alchemy,
                prerequisites: &["gold_synthesis"],
                cost: 2,
            },
            SkillDef {
                id: "philosophers_stone",
                name: "Philosopher's Stone",
                description: "Sell value 100%, rare+ items give 150%",
                branch: SkillBranch::Alchemy,
                prerequisites: &["essence_distill"],
                cost: 2,
            },
            SkillDef {
                id: "material_insight",
                name: "Material Insight",
                description: "+20% GP from Uncommon+ items",
                branch: SkillBranch::Alchemy,
                prerequisites: &["transmute_basics"],
                cost: 3,
            },
            SkillDef {
                id: "catalyst_brew",
                name: "Catalyst Brew",
                description: "Each sell stacks +1% GP bonus (resets on rebirth)",
                branch: SkillBranch::Alchemy,
                prerequisites: &["material_insight"],
                cost: 3,
            },
            SkillDef {
                id: "elixir_of_fortune",
                name: "Elixir of Fortune",
                description: "+5 luck, selling has 10% chance to double GP",
                branch: SkillBranch::Alchemy,
                prerequisites: &["catalyst_brew"],
                cost: 2,
            },
            SkillDef {
                id: "magnum_opus",
                name: "Magnum Opus",
                description: "Capstone: all sell bonuses doubled, +50% GP",
                branch: SkillBranch::Alchemy,
                prerequisites: &["philosophers_stone", "elixir_of_fortune"],
                cost: 5,
            },
            // === Chaos Branch (8) ===
            SkillDef {
                id: "entropy",
                name: "Entropy",
                description: "Loot values vary \u{00b1}30% randomly",
                branch: SkillBranch::Chaos,
                prerequisites: &[],
                cost: 1,
            },
            SkillDef {
                id: "wild_magic",
                name: "Wild Magic",
                description: "5% chance for item to upgrade 1 rarity tier",
                branch: SkillBranch::Chaos,
                prerequisites: &["entropy"],
                cost: 1,
            },
            SkillDef {
                id: "chaos_crit",
                name: "Chaos Crit",
                description: "Crits deal 1x-5x randomly instead of fixed",
                branch: SkillBranch::Chaos,
                prerequisites: &["wild_magic"],
                cost: 2,
            },
            SkillDef {
                id: "reality_tear",
                name: "Reality Tear",
                description: "1% chance for 20x GP on any chest",
                branch: SkillBranch::Chaos,
                prerequisites: &["chaos_crit"],
                cost: 2,
            },
            SkillDef {
                id: "gambler_spirit",
                name: "Gambler's Spirit",
                description: "After 3 bad rolls, next is guaranteed Rare+",
                branch: SkillBranch::Chaos,
                prerequisites: &["entropy"],
                cost: 2,
            },
            SkillDef {
                id: "chaos_surge",
                name: "Chaos Surge",
                description: "Random buff each chest: +50% GP, XP, or Speed for 10s",
                branch: SkillBranch::Chaos,
                prerequisites: &["gambler_spirit"],
                cost: 3,
            },
            SkillDef {
                id: "pandemonium",
                name: "Pandemonium",
                description: "All chaos effects trigger 2x more often",
                branch: SkillBranch::Chaos,
                prerequisites: &["chaos_surge"],
                cost: 3,
            },
            SkillDef {
                id: "singularity",
                name: "Singularity",
                description: "Capstone: 3% chance to triple all loot values",
                branch: SkillBranch::Chaos,
                prerequisites: &["reality_tear", "pandemonium"],
                cost: 5,
            },
        ]
    });
    SKILLS.as_slice()
}

pub fn get_skill(id: &str) -> Option<&'static SkillDef> {
    all_skills().iter().find(|s| s.id == id)
}

pub fn skills_for_branch(branch: SkillBranch) -> Vec<&'static SkillDef> {
    all_skills()
        .iter()
        .filter(|s| s.branch == branch)
        .collect()
}
