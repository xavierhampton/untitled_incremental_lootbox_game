use serde::{Deserialize, Serialize};

use super::item::ItemInstance;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChestType {
    Wooden,
    Iron,
    Silver,
    Gold,
    Crystal,
    Shadow,
    Void,
}

impl ChestType {
    pub const ALL: [ChestType; 7] = [
        ChestType::Wooden,
        ChestType::Iron,
        ChestType::Silver,
        ChestType::Gold,
        ChestType::Crystal,
        ChestType::Shadow,
        ChestType::Void,
    ];

    pub fn index(self) -> usize {
        match self {
            ChestType::Wooden => 0,
            ChestType::Iron => 1,
            ChestType::Silver => 2,
            ChestType::Gold => 3,
            ChestType::Crystal => 4,
            ChestType::Shadow => 5,
            ChestType::Void => 6,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            ChestType::Wooden => "Wooden",
            ChestType::Iron => "Iron",
            ChestType::Silver => "Silver",
            ChestType::Gold => "Gold",
            ChestType::Crystal => "Crystal",
            ChestType::Shadow => "Shadow",
            ChestType::Void => "Void",
        }
    }

    pub fn base_ticks(self) -> u32 {
        match self {
            ChestType::Wooden => 60,   // ~2 seconds
            ChestType::Iron => 90,     // ~3 seconds
            ChestType::Silver => 120,  // ~4 seconds
            ChestType::Gold => 150,    // ~5 seconds
            ChestType::Crystal => 180, // ~6 seconds
            ChestType::Shadow => 240,  // ~8 seconds
            ChestType::Void => 300,    // ~10 seconds
        }
    }

    pub fn required_level(self) -> u32 {
        match self {
            ChestType::Wooden => 1,
            ChestType::Iron => 3,
            ChestType::Silver => 6,
            ChestType::Gold => 10,
            ChestType::Crystal => 15,
            ChestType::Shadow => 22,
            ChestType::Void => 30,
        }
    }

    pub fn color(self) -> ratatui::style::Color {
        match self {
            ChestType::Wooden => ratatui::style::Color::Rgb(139, 90, 43),
            ChestType::Iron => ratatui::style::Color::Gray,
            ChestType::Silver => ratatui::style::Color::White,
            ChestType::Gold => ratatui::style::Color::Yellow,
            ChestType::Crystal => ratatui::style::Color::Cyan,
            ChestType::Shadow => ratatui::style::Color::Magenta,
            ChestType::Void => ratatui::style::Color::Rgb(128, 0, 255),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChestState {
    Idle,
    Opening,
    Revealing,
    Complete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChestProgress {
    pub state: ChestState,
    pub ticks_elapsed: u32,
    pub ticks_required: u32,
    pub reveal_ticks: u32,
    #[serde(skip)]
    pub last_item: Option<ItemInstance>,
}

impl Default for ChestProgress {
    fn default() -> Self {
        Self {
            state: ChestState::Idle,
            ticks_elapsed: 0,
            ticks_required: ChestType::Wooden.base_ticks(),
            reveal_ticks: 0,
            last_item: None,
        }
    }
}

impl ChestProgress {
    pub fn start_opening(&mut self, chest_type: ChestType, speed: f64) {
        self.state = ChestState::Opening;
        self.ticks_elapsed = 0;
        self.ticks_required = (chest_type.base_ticks() as f64 / speed).max(10.0) as u32;
        self.reveal_ticks = 0;
        self.last_item = None;
    }

    pub fn tick(&mut self) {
        match self.state {
            ChestState::Opening => {
                self.ticks_elapsed += 1;
                if self.ticks_elapsed >= self.ticks_required {
                    self.state = ChestState::Revealing;
                    self.reveal_ticks = 0;
                }
            }
            ChestState::Revealing => {
                self.reveal_ticks += 1;
            }
            _ => {}
        }
    }

    pub fn progress_fraction(&self) -> f64 {
        if self.ticks_required == 0 {
            return 0.0;
        }
        (self.ticks_elapsed as f64 / self.ticks_required as f64).min(1.0)
    }

    pub fn collect(&mut self) {
        self.state = ChestState::Idle;
        self.ticks_elapsed = 0;
        self.reveal_ticks = 0;
        self.last_item = None;
    }
}
