use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use ratatui::prelude::*;

#[derive(Debug, Clone)]
struct Particle {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    life: u32,
    max_life: u32,
    color: Color,
    char: char,
}

#[derive(Debug)]
pub struct Firework {
    particles: Vec<Particle>,
    ticks_remaining: u32,
}

impl Firework {
    pub fn new(center_x: f64, center_y: f64, colors: &[Color], count: u32, duration: u32) -> Self {
        let mut rng = SmallRng::from_os_rng();
        let chars = ['*', '.', '+', 'o', '`', '\'', '~', '^', ':', ';'];
        let particles: Vec<Particle> = (0..count)
            .map(|_| {
                let angle: f64 = rng.random::<f64>() * std::f64::consts::TAU;
                let speed: f64 = rng.random::<f64>() * 3.5 + 0.5;
                let life = rng.random_range((duration / 2)..duration);
                let color = colors[rng.random_range(0..colors.len())];
                Particle {
                    x: center_x,
                    y: center_y,
                    vx: angle.cos() * speed,
                    vy: angle.sin() * speed * 0.5,
                    life,
                    max_life: life,
                    color,
                    char: chars[rng.random_range(0..chars.len())],
                }
            })
            .collect();

        Firework {
            particles,
            ticks_remaining: duration,
        }
    }

    pub fn tick(&mut self) {
        self.ticks_remaining = self.ticks_remaining.saturating_sub(1);
        for p in &mut self.particles {
            p.x += p.vx;
            p.y += p.vy;
            p.vy += 0.04;
            p.vx *= 0.98; // air resistance
            p.life = p.life.saturating_sub(1);
        }
        self.particles.retain(|p| p.life > 0);
    }

    pub fn is_done(&self) -> bool {
        self.ticks_remaining == 0 || self.particles.is_empty()
    }

    pub fn render(&self, buf: &mut Buffer, area: Rect) {
        for p in &self.particles {
            let x = p.x as u16;
            let y = p.y as u16;
            if x >= area.x
                && x < area.x + area.width
                && y >= area.y
                && y < area.y + area.height
            {
                if let Some(cell) = buf.cell_mut(Position::new(x, y)) {
                    // Fade character as life decreases
                    let life_frac = p.life as f64 / p.max_life as f64;
                    let ch = if life_frac < 0.2 {
                        '.'
                    } else if life_frac < 0.5 {
                        match p.char {
                            '*' => '+',
                            'o' => '.',
                            _ => p.char,
                        }
                    } else {
                        p.char
                    };
                    cell.set_char(ch);
                    cell.set_fg(p.color);
                }
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct FireworkManager {
    pub fireworks: Vec<Firework>,
}

impl FireworkManager {
    pub fn spawn(&mut self, x: f64, y: f64, colors: &[Color], count: u32, duration: u32) {
        self.fireworks
            .push(Firework::new(x, y, colors, count, duration));
    }

    /// Spawn a burst of multiple fireworks scattered around a center point
    pub fn spawn_burst(
        &mut self,
        cx: f64,
        cy: f64,
        colors: &[Color],
        count_per: u32,
        duration: u32,
        num_bursts: u32,
    ) {
        self.spawn_burst_wide(cx, cy, 12.0, 6.0, colors, count_per, duration, num_bursts);
    }

    /// Spawn a burst with configurable scatter range
    pub fn spawn_burst_wide(
        &mut self,
        cx: f64,
        cy: f64,
        spread_x: f64,
        spread_y: f64,
        colors: &[Color],
        count_per: u32,
        duration: u32,
        num_bursts: u32,
    ) {
        let mut rng = SmallRng::from_os_rng();
        for _ in 0..num_bursts {
            let ox: f64 = rng.random::<f64>() * spread_x - spread_x / 2.0;
            let oy: f64 = rng.random::<f64>() * spread_y - spread_y / 2.0;
            self.spawn(cx + ox, cy + oy, colors, count_per, duration);
        }
    }

    pub fn tick(&mut self) {
        for fw in &mut self.fireworks {
            fw.tick();
        }
        self.fireworks.retain(|fw| !fw.is_done());
    }

    pub fn render(&self, buf: &mut Buffer, area: Rect) {
        for fw in &self.fireworks {
            fw.render(buf, area);
        }
    }
}
