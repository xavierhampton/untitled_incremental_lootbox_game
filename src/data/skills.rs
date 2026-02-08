use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SkillBranch {
    Fortune,
    Speed,
    Mastery,
    Discovery,
}

impl SkillBranch {
    pub const ALL: [SkillBranch; 4] = [
        SkillBranch::Fortune,
        SkillBranch::Speed,
        SkillBranch::Mastery,
        SkillBranch::Discovery,
    ];

    pub fn label(self) -> &'static str {
        match self {
            SkillBranch::Fortune => "Fortune",
            SkillBranch::Speed => "Speed",
            SkillBranch::Mastery => "Mastery",
            SkillBranch::Discovery => "Discovery",
        }
    }

    pub fn color(self) -> ratatui::style::Color {
        match self {
            SkillBranch::Fortune => ratatui::style::Color::Yellow,
            SkillBranch::Speed => ratatui::style::Color::Cyan,
            SkillBranch::Mastery => ratatui::style::Color::Red,
            SkillBranch::Discovery => ratatui::style::Color::Green,
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
}

pub fn all_skills() -> &'static [SkillDef] {
    use std::sync::LazyLock;
    static SKILLS: LazyLock<Vec<SkillDef>> = LazyLock::new(|| {
        vec![
            // === Fortune Branch ===
            SkillDef {
                id: "lucky_charm",
                name: "Lucky Charm",
                description: "+3 luck",
                branch: SkillBranch::Fortune,
                prerequisites: &[],
            },
            SkillDef {
                id: "four_leaf",
                name: "Four-Leaf",
                description: "+5 luck, rare items shimmer",
                branch: SkillBranch::Fortune,
                prerequisites: &["lucky_charm"],
            },
            SkillDef {
                id: "pity_timer",
                name: "Pity Timer",
                description: "After 10 non-rare chests, next is Rare+",
                branch: SkillBranch::Fortune,
                prerequisites: &["four_leaf"],
            },
            SkillDef {
                id: "golden_touch",
                name: "Golden Touch",
                description: "15% chance for double GP from a chest",
                branch: SkillBranch::Fortune,
                prerequisites: &["pity_timer"],
            },
            SkillDef {
                id: "treasure_sense",
                name: "Treasure Sense",
                description: "+50% GP from all sources",
                branch: SkillBranch::Fortune,
                prerequisites: &["lucky_charm"],
            },
            SkillDef {
                id: "double_or_nothing",
                name: "Double or Nothing",
                description: "20% double loot, 10% nothing",
                branch: SkillBranch::Fortune,
                prerequisites: &["treasure_sense"],
            },
            SkillDef {
                id: "jackpot",
                name: "Jackpot",
                description: "Every 50th chest is Epic+",
                branch: SkillBranch::Fortune,
                prerequisites: &["double_or_nothing"],
            },
            // === Speed Branch ===
            SkillDef {
                id: "swift_hands",
                name: "Swift Hands",
                description: "+30% open speed",
                branch: SkillBranch::Speed,
                prerequisites: &[],
            },
            SkillDef {
                id: "nimble_fingers",
                name: "Nimble Fingers",
                description: "+20% open speed (stacks)",
                branch: SkillBranch::Speed,
                prerequisites: &["swift_hands"],
            },
            SkillDef {
                id: "chain_opener",
                name: "Chain Opener",
                description: "Opening a chest instantly starts the next",
                branch: SkillBranch::Speed,
                prerequisites: &["nimble_fingers"],
            },
            SkillDef {
                id: "quick_collect",
                name: "Quick Collect",
                description: "Auto-collect loot after 1 second",
                branch: SkillBranch::Speed,
                prerequisites: &["chain_opener"],
            },
            SkillDef {
                id: "auto_opener",
                name: "Auto Opener",
                description: "Chests open automatically (50% speed)",
                branch: SkillBranch::Speed,
                prerequisites: &["swift_hands"],
            },
            SkillDef {
                id: "idle_income",
                name: "Idle Income",
                description: "Earn 1 GP/sec while not opening chests",
                branch: SkillBranch::Speed,
                prerequisites: &["auto_opener"],
            },
            SkillDef {
                id: "perpetual_motion",
                name: "Perpetual Motion",
                description: "Auto opener runs at 100% speed",
                branch: SkillBranch::Speed,
                prerequisites: &["idle_income"],
            },
            // === Mastery Branch ===
            SkillDef {
                id: "critical_eye",
                name: "Critical Eye",
                description: "+5% crit chance",
                branch: SkillBranch::Mastery,
                prerequisites: &[],
            },
            SkillDef {
                id: "crit_cascade",
                name: "Crit Cascade",
                description: "Crits have 25% chance to trigger another",
                branch: SkillBranch::Mastery,
                prerequisites: &["critical_eye"],
            },
            SkillDef {
                id: "multi_drop",
                name: "Multi-Drop",
                description: "10% chance to find 2 items from one chest",
                branch: SkillBranch::Mastery,
                prerequisites: &["crit_cascade"],
            },
            SkillDef {
                id: "legendary_aura",
                name: "Legendary Aura",
                description: "Legendary items give 3x XP",
                branch: SkillBranch::Mastery,
                prerequisites: &["multi_drop"],
            },
            SkillDef {
                id: "overcharge",
                name: "Overcharge",
                description: "Crit multiplier: 2x \u{2192} 3x",
                branch: SkillBranch::Mastery,
                prerequisites: &["critical_eye"],
            },
            SkillDef {
                id: "xp_surge",
                name: "XP Surge",
                description: "Every 5th chest gives 5x XP",
                branch: SkillBranch::Mastery,
                prerequisites: &["overcharge"],
            },
            SkillDef {
                id: "midas_touch",
                name: "Midas Touch",
                description: "+100% base GP on items",
                branch: SkillBranch::Mastery,
                prerequisites: &["xp_surge"],
            },
            // === Discovery Branch ===
            SkillDef {
                id: "iron_key",
                name: "Iron Key",
                description: "Unlock Iron chests",
                branch: SkillBranch::Discovery,
                prerequisites: &[],
            },
            SkillDef {
                id: "silver_key",
                name: "Silver Key",
                description: "Unlock Silver chests",
                branch: SkillBranch::Discovery,
                prerequisites: &["iron_key"],
            },
            SkillDef {
                id: "gold_key",
                name: "Gold Key",
                description: "Unlock Gold chests",
                branch: SkillBranch::Discovery,
                prerequisites: &["silver_key"],
            },
            SkillDef {
                id: "void_attune",
                name: "Void Attune",
                description: "Unlock Crystal, Shadow, and Void chests",
                branch: SkillBranch::Discovery,
                prerequisites: &["gold_key"],
            },
            SkillDef {
                id: "scavenger",
                name: "Scavenger",
                description: "5% chance for a bonus Common item",
                branch: SkillBranch::Discovery,
                prerequisites: &["iron_key"],
            },
            SkillDef {
                id: "relic_hunter",
                name: "Relic Hunter",
                description: "Double relic drop chance",
                branch: SkillBranch::Discovery,
                prerequisites: &["scavenger"],
            },
            SkillDef {
                id: "recycler",
                name: "Recycler",
                description: "Auto-sell Common items for GP",
                branch: SkillBranch::Discovery,
                prerequisites: &["relic_hunter"],
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
