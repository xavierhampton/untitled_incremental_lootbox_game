#[cfg(not(target_arch = "wasm32"))]
mod sounds;
#[cfg(not(target_arch = "wasm32"))]
pub use sounds::SoundManager;

#[cfg(target_arch = "wasm32")]
mod sounds_stub;
#[cfg(target_arch = "wasm32")]
pub use sounds_stub::SoundManager;
