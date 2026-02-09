use std::time::{Duration, Instant};

use color_eyre::Result;
use crossterm::event::{self, KeyEvent};

pub enum Event {
    Tick,
    Key(KeyEvent),
}

pub struct EventHandler {
    tick_rate: Duration,
    last_tick: Instant,
}

impl EventHandler {
    pub fn new(tick_rate_ms: u64) -> Self {
        Self {
            tick_rate: Duration::from_millis(tick_rate_ms),
            last_tick: Instant::now(),
        }
    }

    pub fn next(&mut self) -> Result<Event> {
        loop {
            // Check for input without blocking
            if event::poll(Duration::from_millis(0))? {
                if let event::Event::Key(key) = event::read()? {
                    if key.kind == event::KeyEventKind::Press {
                        return Ok(Event::Key(key));
                    }
                }
            }

            // Check if it's time for a tick
            let now = Instant::now();
            let elapsed = now.duration_since(self.last_tick);

            if elapsed >= self.tick_rate {
                self.last_tick = now;
                return Ok(Event::Tick);
            }

            // Calculate how long to sleep (remaining time until next tick)
            let remaining = self.tick_rate.saturating_sub(elapsed);
            let sleep_time = remaining.min(Duration::from_millis(5));

            // Sleep for a short time to avoid busy-waiting
            std::thread::sleep(sleep_time);
        }
    }
}
