use crate::game::item::Rarity;
use web_sys::{AudioContext, OscillatorType};

pub struct SoundManager {
    ctx: Option<AudioContext>,
    volume: f32,
    last_play_ms: f64,
}

const MIN_SOUND_GAP_MS: f64 = 60.0;

impl SoundManager {
    pub fn new() -> Option<Self> {
        // Don't create AudioContext here — browsers block it until user gesture.
        // It will be created lazily on first sound request (which happens on keypress).
        Some(Self {
            ctx: None,
            volume: 0.8,
            last_play_ms: f64::NEG_INFINITY,
        })
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0).powf(3.0);
    }

    /// Get or create the AudioContext (must be called during a user gesture)
    /// and check the throttle. Returns the context if we're allowed to play.
    fn acquire(&mut self) -> Option<&AudioContext> {
        if self.ctx.is_none() {
            self.ctx = AudioContext::new().ok();
        }
        let ctx = self.ctx.as_ref()?;

        // Resume if suspended (handles browser autoplay policy / tab-switch)
        if ctx.state() == web_sys::AudioContextState::Suspended {
            let _ = ctx.resume();
        }

        // Throttle: skip if last sound was too recent
        let now = ctx.current_time() * 1000.0;
        if now - self.last_play_ms < MIN_SOUND_GAP_MS {
            return None;
        }
        self.last_play_ms = now;

        Some(ctx)
    }

    /// Play a tone: frequency in Hz, start offset in seconds, duration in seconds.
    fn schedule_tone(ctx: &AudioContext, freq: f32, start: f64, duration: f64, gain_value: f32) {
        let osc = match ctx.create_oscillator() {
            Ok(o) => o,
            Err(_) => return,
        };
        let gain = match ctx.create_gain() {
            Ok(g) => g,
            Err(_) => return,
        };

        osc.set_type(OscillatorType::Sine);
        osc.frequency().set_value(freq);

        // Envelope: quick attack, sustain, quick release
        let end = start + duration;
        let g = gain.gain();
        let _ = g.set_value_at_time(0.0, start);
        let _ = g.linear_ramp_to_value_at_time(gain_value, start + 0.003);
        if duration > 0.008 {
            let _ = g.set_value_at_time(gain_value, end - 0.005);
            let _ = g.linear_ramp_to_value_at_time(0.0, end);
        }

        let _ = osc.connect_with_audio_node(&gain);
        let _ = gain.connect_with_audio_node(&ctx.destination());

        let _ = osc.start_with_when(start);
        let _ = osc.stop_with_when(end);
    }

    /// Play a sequence of notes (freq_hz, duration_ms) one after another.
    fn play_notes(&mut self, notes: &[(f32, f64)]) {
        if self.volume <= 0.0 {
            return;
        }
        let gain = self.volume * 0.3;
        let Some(ctx) = self.acquire() else { return };
        let mut t = ctx.current_time();
        for &(freq, dur_ms) in notes {
            let dur = dur_ms / 1000.0;
            Self::schedule_tone(ctx, freq, t, dur, gain);
            t += dur;
        }
    }

    /// Play a frequency sweep from start_freq to end_freq.
    fn play_sweep(&mut self, start_freq: f32, end_freq: f32, dur_ms: f64) {
        if self.volume <= 0.0 {
            return;
        }
        let gain_value = self.volume * 0.3;
        let Some(ctx) = self.acquire() else { return };
        let dur = dur_ms / 1000.0;
        let t = ctx.current_time();

        let osc = match ctx.create_oscillator() {
            Ok(o) => o,
            Err(_) => return,
        };
        let gain = match ctx.create_gain() {
            Ok(g) => g,
            Err(_) => return,
        };

        osc.set_type(OscillatorType::Sine);
        let _ = osc.frequency().set_value_at_time(start_freq, t);
        let _ = osc.frequency().linear_ramp_to_value_at_time(end_freq, t + dur);

        let end = t + dur;
        let g = gain.gain();
        let _ = g.set_value_at_time(0.0, t);
        let _ = g.linear_ramp_to_value_at_time(gain_value, t + 0.003);
        if dur > 0.008 {
            let _ = g.set_value_at_time(gain_value, end - 0.005);
            let _ = g.linear_ramp_to_value_at_time(0.0, end);
        }

        let _ = osc.connect_with_audio_node(&gain);
        let _ = gain.connect_with_audio_node(&ctx.destination());

        let _ = osc.start_with_when(t);
        let _ = osc.stop_with_when(end);
    }

    /// Play multiple note sequences simultaneously (for chords).
    fn play_layered(&mut self, layers: &[Vec<(f32, f64)>]) {
        if self.volume <= 0.0 || layers.is_empty() {
            return;
        }
        let layer_gain = self.volume * 0.3 / layers.len() as f32;
        let Some(ctx) = self.acquire() else { return };
        for notes in layers {
            let mut t = ctx.current_time();
            for &(freq, dur_ms) in notes {
                let dur = dur_ms / 1000.0;
                Self::schedule_tone(ctx, freq, t, dur, layer_gain);
                t += dur;
            }
        }
    }

    // --- Sound effects (matching native sounds.rs) ---

    pub fn play_click(&mut self) {
        self.play_notes(&[(1200.0, 25.0)]);
    }

    pub fn play_tab_switch(&mut self) {
        self.play_sweep(600.0, 800.0, 60.0);
    }

    pub fn play_menu_open(&mut self) {
        self.play_notes(&[(400.0, 50.0), (600.0, 50.0)]);
    }

    pub fn play_menu_close(&mut self) {
        self.play_notes(&[(600.0, 50.0), (400.0, 50.0)]);
    }

    pub fn play_error(&mut self) {
        self.play_notes(&[(200.0, 80.0)]);
    }

    pub fn play_chest_start(&mut self) {
        self.play_sweep(300.0, 500.0, 150.0);
    }

    pub fn play_reveal(&mut self, rarity: Rarity) {
        match rarity {
            Rarity::Common => {
                self.play_notes(&[(523.0, 100.0)]);
            }
            Rarity::Uncommon => {
                self.play_notes(&[(523.0, 80.0), (659.0, 100.0)]);
            }
            Rarity::Rare => {
                self.play_notes(&[(523.0, 70.0), (659.0, 70.0), (784.0, 100.0)]);
            }
            Rarity::Epic => {
                self.play_notes(&[
                    (523.0, 60.0),
                    (659.0, 60.0),
                    (784.0, 60.0),
                    (1047.0, 120.0),
                ]);
            }
            Rarity::Legendary => {
                self.play_notes(&[
                    (523.0, 50.0),
                    (659.0, 50.0),
                    (784.0, 50.0),
                    (1047.0, 50.0),
                    (1319.0, 180.0),
                ]);
            }
            Rarity::Mythic => {
                self.play_layered(&[
                    vec![(523.0, 300.0)],
                    vec![(659.0, 300.0)],
                    vec![
                        (1047.0, 60.0),
                        (1319.0, 60.0),
                        (1568.0, 60.0),
                        (2093.0, 200.0),
                    ],
                ]);
            }
        }
    }

    pub fn play_crit(&mut self) {
        self.play_notes(&[(2000.0, 40.0)]);
    }

    pub fn play_collect(&mut self) {
        self.play_sweep(800.0, 1200.0, 50.0);
    }

    pub fn play_level_up(&mut self) {
        self.play_notes(&[
            (523.0, 60.0),
            (587.0, 60.0),
            (659.0, 60.0),
            (698.0, 60.0),
            (784.0, 120.0),
        ]);
    }

    pub fn play_purchase(&mut self) {
        self.play_notes(&[(1500.0, 30.0), (2000.0, 50.0)]);
    }

    pub fn play_sell(&mut self) {
        self.play_sweep(1000.0, 600.0, 80.0);
    }

    pub fn play_rebirth(&mut self) {
        self.play_sweep(150.0, 400.0, 300.0);
    }
}
