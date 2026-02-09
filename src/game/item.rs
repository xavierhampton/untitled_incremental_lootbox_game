use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
}

impl Rarity {
    pub fn color(self) -> ratatui::style::Color {
        match self {
            Rarity::Common => ratatui::style::Color::Gray,
            Rarity::Uncommon => ratatui::style::Color::Green,
            Rarity::Rare => ratatui::style::Color::Blue,
            Rarity::Epic => ratatui::style::Color::Magenta,
            Rarity::Legendary => ratatui::style::Color::Yellow,
            Rarity::Mythic => ratatui::style::Color::Rgb(255, 50, 50),
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Rarity::Common => "Common",
            Rarity::Uncommon => "Uncommon",
            Rarity::Rare => "Rare",
            Rarity::Epic => "Epic",
            Rarity::Legendary => "Legendary",
            Rarity::Mythic => "Mythic",
        }
    }

    pub fn gp_multiplier(self) -> f64 {
        match self {
            Rarity::Common => 1.0,
            Rarity::Uncommon => 2.5,
            Rarity::Rare => 6.0,
            Rarity::Epic => 15.0,
            Rarity::Legendary => 50.0,
            Rarity::Mythic => 150.0,
        }
    }

    pub fn xp_multiplier(self) -> f64 {
        match self {
            Rarity::Common => 1.0,
            Rarity::Uncommon => 2.0,
            Rarity::Rare => 4.0,
            Rarity::Epic => 10.0,
            Rarity::Legendary => 25.0,
            Rarity::Mythic => 60.0,
        }
    }
}

pub type ItemId = &'static str;

#[derive(Debug, Clone)]
pub struct ItemDef {
    pub id: ItemId,
    pub name: &'static str,
    pub rarity: Rarity,
    pub base_gp: u64,
    pub base_xp: u64,
    pub description: &'static str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemInstance {
    pub id: String,
    pub name: String,
    pub rarity: Rarity,
    pub gp_value: u64,
    pub xp_value: u64,
    pub is_crit: bool,
}
