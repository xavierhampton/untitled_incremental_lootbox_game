use super::state::GameState;

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use super::GameState;
    use directories::ProjectDirs;
    use std::fs;
    use std::path::PathBuf;

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
}

#[cfg(target_arch = "wasm32")]
mod web {
    use super::GameState;

    const STORAGE_KEY: &str = "lootbox_game_save";

    fn local_storage() -> Option<web_sys::Storage> {
        web_sys::window()?.local_storage().ok()?
    }

    pub fn save_game(state: &GameState) {
        if let Some(storage) = local_storage() {
            if let Ok(json) = serde_json::to_string(state) {
                let _ = storage.set_item(STORAGE_KEY, &json);
            }
        }
    }

    pub fn load_game() -> Option<GameState> {
        let storage = local_storage()?;
        let data = storage.get_item(STORAGE_KEY).ok()??;
        serde_json::from_str(&data).ok()
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::{load_game, save_game};

#[cfg(target_arch = "wasm32")]
pub use web::{load_game, save_game};
