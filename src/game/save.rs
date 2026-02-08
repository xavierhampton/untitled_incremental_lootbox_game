use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

use super::state::GameState;

fn save_path() -> Option<PathBuf> {
    ProjectDirs::from("", "", "lootbox-game").map(|dirs| {
        let data_dir = dirs.data_dir();
        fs::create_dir_all(data_dir).ok();
        data_dir.join("save.json")
    })
}

pub fn save_game(state: &GameState) {
    if let Some(path) = save_path() {
        if let Ok(json) = serde_json::to_string_pretty(state) {
            let _ = fs::write(path, json);
        }
    }
}

pub fn load_game() -> Option<GameState> {
    let path = save_path()?;
    let data = fs::read_to_string(path).ok()?;
    serde_json::from_str(&data).ok()
}
