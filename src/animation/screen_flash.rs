use ratatui::prelude::*;

#[derive(Debug)]
pub struct ScreenFlash {
    pub ticks_remaining: u32,
    pub total_ticks: u32,
    pub color: Color,
}

impl ScreenFlash {
    pub fn new(color: Color, duration_ticks: u32) -> Self {
        Self {
            ticks_remaining: duration_ticks,
            total_ticks: duration_ticks,
            color,
        }
    }

    pub fn tick(&mut self) {
        self.ticks_remaining = self.ticks_remaining.saturating_sub(1);
    }

    pub fn is_done(&self) -> bool {
        self.ticks_remaining == 0
    }

    pub fn render(&self, buf: &mut Buffer, area: Rect) {
        if self.is_done() {
            return;
        }
        let intensity = self.ticks_remaining as f64 / self.total_ticks as f64;
        if intensity < 0.3 {
            return;
        }

        // Overlay the flash color on all cells
        for y in area.y..area.y + area.height {
            for x in area.x..area.x + area.width {
                if let Some(cell) = buf.cell_mut(Position::new(x, y)) {
                    cell.set_bg(self.color);
                }
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct FlashManager {
    pub flashes: Vec<ScreenFlash>,
}

impl FlashManager {
    pub fn spawn(&mut self, color: Color, duration: u32) {
        self.flashes.push(ScreenFlash::new(color, duration));
    }

    pub fn tick(&mut self) {
        for f in &mut self.flashes {
            f.tick();
        }
        self.flashes.retain(|f| !f.is_done());
    }

    pub fn render(&self, buf: &mut Buffer, area: Rect) {
        for f in &self.flashes {
            f.render(buf, area);
        }
    }
}
