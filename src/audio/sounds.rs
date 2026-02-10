use std::time::{Duration, Instant};

use rodio::{OutputStream, OutputStreamHandle, Sink, Source};

use crate::game::item::Rarity;

pub struct SoundManager {
    _stream: OutputStream,
    handle: OutputStreamHandle,
    volume: f32,
    last_play: Instant,
}

/// Minimum gap between any two sounds to prevent stacking when keys are held.
const MIN_SOUND_GAP: Duration = Duration::from_millis(60);

/// A simple sine wave source for procedural sound generation.
struct SineWave {
    freq: f32,
    sample_rate: u32,
    sample_idx: u64,
    duration_samples: u64,
}

impl SineWave {
    fn new(freq: f32, duration: Duration) -> Self {
        let sample_rate = 44100;
        let duration_samples = (sample_rate as f64 * duration.as_secs_f64()) as u64;
        Self {
            freq,
            sample_rate,
            sample_idx: 0,
            duration_samples,
        }
    }
}

impl Iterator for SineWave {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if self.sample_idx >= self.duration_samples {
            return None;
        }
        let t = self.sample_idx as f32 / self.sample_rate as f32;
        let sample = (2.0 * std::f32::consts::PI * self.freq * t).sin();
        // Apply a quick fade-out envelope to avoid clicks
        let remaining = (self.duration_samples - self.sample_idx) as f32;
        let fade_samples = (self.sample_rate as f32 * 0.005).min(remaining); // 5ms fade
        let envelope = if remaining < fade_samples {
            remaining / fade_samples
        } else {
            // Also apply attack envelope
            let attack_samples = self.sample_rate as f32 * 0.003; // 3ms attack
            if (self.sample_idx as f32) < attack_samples {
                self.sample_idx as f32 / attack_samples
            } else {
                1.0
            }
        };
        self.sample_idx += 1;
        Some(sample * envelope * 0.3) // 0.3 base amplitude to avoid clipping
    }
}

impl Source for SineWave {
    fn current_frame_len(&self) -> Option<usize> {
        let remaining = self.duration_samples.saturating_sub(self.sample_idx) as usize;
        if remaining == 0 { None } else { Some(remaining) }
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_secs_f64(
            self.duration_samples as f64 / self.sample_rate as f64,
        ))
    }
}

/// A frequency sweep source (linear interpolation from start to end frequency).
struct FreqSweep {
    start_freq: f32,
    end_freq: f32,
    sample_rate: u32,
    sample_idx: u64,
    duration_samples: u64,
    phase: f64,
}

impl FreqSweep {
    fn new(start_freq: f32, end_freq: f32, duration: Duration) -> Self {
        let sample_rate = 44100;
        let duration_samples = (sample_rate as f64 * duration.as_secs_f64()) as u64;
        Self {
            start_freq,
            end_freq,
            sample_rate,
            sample_idx: 0,
            duration_samples,
            phase: 0.0,
        }
    }
}

impl Iterator for FreqSweep {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if self.sample_idx >= self.duration_samples {
            return None;
        }
        let t = self.sample_idx as f64 / self.duration_samples as f64;
        let freq = self.start_freq as f64 + (self.end_freq as f64 - self.start_freq as f64) * t;
        self.phase += 2.0 * std::f64::consts::PI * freq / self.sample_rate as f64;
        let sample = self.phase.sin() as f32;

        // Envelope
        let remaining = (self.duration_samples - self.sample_idx) as f32;
        let fade_samples = (self.sample_rate as f32 * 0.005).min(remaining);
        let envelope = if remaining < fade_samples {
            remaining / fade_samples
        } else {
            let attack_samples = self.sample_rate as f32 * 0.003;
            if (self.sample_idx as f32) < attack_samples {
                self.sample_idx as f32 / attack_samples
            } else {
                1.0
            }
        };
        self.sample_idx += 1;
        Some(sample * envelope * 0.3)
    }
}

impl Source for FreqSweep {
    fn current_frame_len(&self) -> Option<usize> {
        let remaining = self.duration_samples.saturating_sub(self.sample_idx) as usize;
        if remaining == 0 { None } else { Some(remaining) }
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_secs_f64(
            self.duration_samples as f64 / self.sample_rate as f64,
        ))
    }
}

impl SoundManager {
    pub fn new() -> Option<Self> {
        let (stream, handle) = OutputStream::try_default().ok()?;
        Some(Self {
            _stream: stream,
            handle,
            volume: 0.8,
            last_play: Instant::now() - Duration::from_secs(1),
        })
    }

    pub fn set_volume(&mut self, volume: f32) {
        // Apply power curve: slider 50% → ~12.5% actual volume
        // This makes the quiet end of the slider much more usable
        self.volume = volume.clamp(0.0, 1.0).powf(3.0);
    }

    /// Returns false (and skips) if a sound was played too recently.
    fn throttle(&mut self) -> bool {
        let now = Instant::now();
        if now.duration_since(self.last_play) < MIN_SOUND_GAP {
            return false;
        }
        self.last_play = now;
        true
    }

    /// Play a sequence of sine tones with specified frequencies and durations.
    fn play_notes(&mut self, notes: &[(f32, Duration)]) {
        if self.volume <= 0.0 || !self.throttle() {
            return;
        }
        let sink = match Sink::try_new(&self.handle) {
            Ok(s) => s,
            Err(_) => return,
        };
        sink.set_volume(self.volume);
        for &(freq, dur) in notes {
            sink.append(SineWave::new(freq, dur));
        }
        sink.detach();
    }

    /// Play a frequency sweep.
    fn play_sweep(&mut self, start: f32, end: f32, duration: Duration) {
        if self.volume <= 0.0 || !self.throttle() {
            return;
        }
        let sink = match Sink::try_new(&self.handle) {
            Ok(s) => s,
            Err(_) => return,
        };
        sink.set_volume(self.volume);
        sink.append(FreqSweep::new(start, end, duration));
        sink.detach();
    }

    /// Play multiple note sequences simultaneously (for chords).
    /// Volume is divided by layer count so stacking doesn't clip.
    fn play_layered(&mut self, layers: &[Vec<(f32, Duration)>]) {
        if self.volume <= 0.0 || layers.is_empty() || !self.throttle() {
            return;
        }
        let layer_volume = self.volume / layers.len() as f32;
        for notes in layers {
            let sink = match Sink::try_new(&self.handle) {
                Ok(s) => s,
                Err(_) => continue,
            };
            sink.set_volume(layer_volume);
            for &(freq, dur) in notes {
                sink.append(SineWave::new(freq, dur));
            }
            sink.detach();
        }
    }

    // --- Sound effects ---

    /// Quick blip for UI clicks / space press
    pub fn play_click(&mut self) {
        self.play_notes(&[(1200.0, Duration::from_millis(25))]);
    }

    /// Two-note sweep for tab switching
    pub fn play_tab_switch(&mut self) {
        self.play_sweep(600.0, 800.0, Duration::from_millis(60));
    }

    /// Ascending pair for menu open
    pub fn play_menu_open(&mut self) {
        self.play_notes(&[
            (400.0, Duration::from_millis(50)),
            (600.0, Duration::from_millis(50)),
        ]);
    }

    /// Descending pair for menu close
    pub fn play_menu_close(&mut self) {
        self.play_notes(&[
            (600.0, Duration::from_millis(50)),
            (400.0, Duration::from_millis(50)),
        ]);
    }

    /// Low buzz for errors / locked actions
    pub fn play_error(&mut self) {
        self.play_notes(&[(200.0, Duration::from_millis(80))]);
    }

    /// Rising sweep for chest opening start
    pub fn play_chest_start(&mut self) {
        self.play_sweep(300.0, 500.0, Duration::from_millis(150));
    }

    /// Reveal sound scaled by rarity — same volume, more notes/higher pitch for rarer
    pub fn play_reveal(&mut self, rarity: Rarity) {
        let ms = Duration::from_millis;
        match rarity {
            Rarity::Common => {
                // Simple ding: C5
                self.play_notes(&[(523.0, ms(100))]);
            }
            Rarity::Uncommon => {
                // Two ascending notes: C5 -> E5
                self.play_notes(&[(523.0, ms(80)), (659.0, ms(100))]);
            }
            Rarity::Rare => {
                // Three-note arpeggio: C5 -> E5 -> G5
                self.play_notes(&[
                    (523.0, ms(70)),
                    (659.0, ms(70)),
                    (784.0, ms(100)),
                ]);
            }
            Rarity::Epic => {
                // Four-note fanfare: C5 -> E5 -> G5 -> C6
                self.play_notes(&[
                    (523.0, ms(60)),
                    (659.0, ms(60)),
                    (784.0, ms(60)),
                    (1047.0, ms(120)),
                ]);
            }
            Rarity::Legendary => {
                // Five-note arpeggio climbing higher: C5 -> E5 -> G5 -> C6 -> E6
                self.play_notes(&[
                    (523.0, ms(50)),
                    (659.0, ms(50)),
                    (784.0, ms(50)),
                    (1047.0, ms(50)),
                    (1319.0, ms(180)),
                ]);
            }
            Rarity::Mythic => {
                // Grand fanfare: chord + high arpeggio (volume split across layers)
                self.play_layered(&[
                    // Soft chord pad
                    vec![(523.0, ms(300))],
                    vec![(659.0, ms(300))],
                    // High arpeggio on top
                    vec![
                        (1047.0, ms(60)),
                        (1319.0, ms(60)),
                        (1568.0, ms(60)),
                        (2093.0, ms(200)),
                    ],
                ]);
            }
        }
    }

    /// Extra sparkle for crit hits (high shimmer)
    pub fn play_crit(&mut self) {
        self.play_notes(&[(2000.0, Duration::from_millis(40))]);
    }

    /// Quick pickup sound for collecting loot
    pub fn play_collect(&mut self) {
        self.play_sweep(800.0, 1200.0, Duration::from_millis(50));
    }

    /// Ascending scale for level up: C-D-E-F-G
    pub fn play_level_up(&mut self) {
        let ms = Duration::from_millis;
        self.play_notes(&[
            (523.0, ms(60)),  // C5
            (587.0, ms(60)),  // D5
            (659.0, ms(60)),  // E5
            (698.0, ms(60)),  // F5
            (784.0, ms(120)), // G5
        ]);
    }

    /// Ka-ching sound for purchases
    pub fn play_purchase(&mut self) {
        let ms = Duration::from_millis;
        self.play_notes(&[
            (1500.0, ms(30)),
            (2000.0, ms(50)),
        ]);
    }

    /// Coin drop sound for selling
    pub fn play_sell(&mut self) {
        self.play_sweep(1000.0, 600.0, Duration::from_millis(80));
    }

    /// Deep transformation sweep for rebirth
    pub fn play_rebirth(&mut self) {
        self.play_sweep(150.0, 400.0, Duration::from_millis(300));
    }
}
