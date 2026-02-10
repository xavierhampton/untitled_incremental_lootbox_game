pub fn xp_for_level(level: u32) -> u64 {
    (100.0 * (level as f64).powf(2.0)) as u64
}
