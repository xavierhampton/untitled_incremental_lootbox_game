#[derive(Debug, Clone)]
pub struct UpgradeDef {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub max_level: u32,
    pub base_cost: u64,
    pub cost_scaling: f64,
    pub category: UpgradeCategory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpgradeCategory {
    Speed,
    Luck,
    Wealth,
    Unlock,
}

impl UpgradeCategory {
    pub fn label(self) -> &'static str {
        match self {
            UpgradeCategory::Speed => "Speed",
            UpgradeCategory::Luck => "Luck",
            UpgradeCategory::Wealth => "Wealth",
            UpgradeCategory::Unlock => "Unlock",
        }
    }
}

impl UpgradeDef {
    pub fn cost_at_level(&self, level: u32) -> u64 {
        (self.base_cost as f64 * self.cost_scaling.powi(level as i32)) as u64
    }
}

pub fn all_upgrades() -> Vec<UpgradeDef> {
    vec![
        // Speed tree
        UpgradeDef {
            id: "swift_hands",
            name: "Swift Hands",
            description: "+10% open speed per level",
            max_level: 10,
            base_cost: 50,
            cost_scaling: 1.5,
            category: UpgradeCategory::Speed,
        },
        UpgradeDef {
            id: "nimble_fingers",
            name: "Nimble Fingers",
            description: "+5% open speed per level",
            max_level: 20,
            base_cost: 30,
            cost_scaling: 1.3,
            category: UpgradeCategory::Speed,
        },
        // Luck tree
        UpgradeDef {
            id: "lucky_charm",
            name: "Lucky Charm",
            description: "+1 luck per level",
            max_level: 15,
            base_cost: 40,
            cost_scaling: 1.4,
            category: UpgradeCategory::Luck,
        },
        UpgradeDef {
            id: "four_leaf",
            name: "Four-Leaf Clover",
            description: "+2 luck per level",
            max_level: 10,
            base_cost: 100,
            cost_scaling: 1.6,
            category: UpgradeCategory::Luck,
        },
        UpgradeDef {
            id: "critical_eye",
            name: "Critical Eye",
            description: "+2% crit chance per level",
            max_level: 10,
            base_cost: 80,
            cost_scaling: 1.5,
            category: UpgradeCategory::Luck,
        },

        // Wealth tree
        UpgradeDef {
            id: "gold_touch",
            name: "Gold Touch",
            description: "+10% GP multiplier per level",
            max_level: 15,
            base_cost: 60,
            cost_scaling: 1.4,
            category: UpgradeCategory::Wealth,
        },
        UpgradeDef {
            id: "xp_boost",
            name: "XP Boost",
            description: "+10% XP multiplier per level",
            max_level: 15,
            base_cost: 60,
            cost_scaling: 1.4,
            category: UpgradeCategory::Wealth,
        },
        UpgradeDef {
            id: "treasure_sense",
            name: "Treasure Sense",
            description: "+20% GP multiplier per level",
            max_level: 10,
            base_cost: 200,
            cost_scaling: 1.7,
            category: UpgradeCategory::Wealth,
        },

    ]
}

pub fn get_upgrade(id: &str) -> Option<&'static UpgradeDef> {
    use std::sync::LazyLock;
    static UPGRADES: LazyLock<Vec<UpgradeDef>> = LazyLock::new(all_upgrades);
    UPGRADES.iter().find(|u| u.id == id)
}
