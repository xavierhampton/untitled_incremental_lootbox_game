use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::app::App;
use super::game_view;
use super::tab_panel;

pub fn draw_layout(frame: &mut Frame, app: &App) {
    let size = frame.area();

    // Main vertical split: content + footer
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(1)])
        .split(size);

    // Horizontal split: 45% game view, 55% tab panel
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
        .split(outer[0]);

    // Draw game view (left)
    game_view::draw(frame, app, columns[0]);

    // Draw tab panel (right)
    tab_panel::draw(frame, app, columns[1]);

    // Footer with controls
    let footer_text = " [Space] Open  [\u{2190}/\u{2192}] Tabs  [1-7] Chest  [Enter] Buy/Learn  [E] Equip  [S] Sell  [R] Rebirth  [?] Help  [Q] Quit";
    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(footer, outer[1]);

    // Help overlay
    if app.show_help {
        draw_help_overlay(frame, size);
    }

    // Float texts overlay
    draw_float_texts(frame, app, columns[0]);

    // Firework and flash overlays (rendered directly to buffer)
    let buf = frame.buffer_mut();
    app.flashes.render(buf, size);
    app.fireworks.render(buf, size);
}

fn draw_help_overlay(frame: &mut Frame, area: Rect) {
    let overlay_width = 50.min(area.width.saturating_sub(4));
    let overlay_height = 16.min(area.height.saturating_sub(4));
    let x = (area.width.saturating_sub(overlay_width)) / 2;
    let y = (area.height.saturating_sub(overlay_height)) / 2;
    let overlay_area = Rect::new(x, y, overlay_width, overlay_height);

    // Clear background
    let clear = ratatui::widgets::Clear;
    frame.render_widget(clear, overlay_area);

    let help_text = vec![
        Line::from(Span::styled("Controls", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(" Space        Open chest / Collect"),
        Line::from(" Enter        Buy upgrade / Learn skill"),
        Line::from(" Left/Right   Switch tabs"),
        Line::from(" 1-7          Select chest type"),
        Line::from(" Up/Down      Scroll list"),
        Line::from(" E            Equip/unequip relic"),
        Line::from(" Q            Quit (auto-saves)"),
        Line::from(" ?            Toggle this help"),
        Line::from(""),
        Line::from(Span::styled("Open chests, collect loot, buy upgrades!", Style::default().fg(Color::Gray))),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow))
        .title(" Help ");
    let paragraph = Paragraph::new(help_text).block(block);
    frame.render_widget(paragraph, overlay_area);
}

fn draw_float_texts(frame: &mut Frame, app: &App, area: Rect) {
    use crate::app::FloatDir;

    for (i, ft) in app.float_texts.iter().enumerate() {
        let progress = 1.0 - (ft.ticks_remaining as f64 / ft.total_ticks as f64);
        let stagger = (i as u16 % 3) as i16;

        // Compute position based on direction
        let start_cx = area.x as i16 + (area.width / 2) as i16 + ft.x_offset;
        let start_cy = area.y as i16 + (area.height * 2 / 3) as i16;

        let (fx, fy) = match ft.dir {
            FloatDir::Up => {
                let y_travel = (progress * 10.0) as i16;
                (start_cx, start_cy - y_travel + stagger)
            }
            FloatDir::Left => {
                let x_travel = (progress * (area.width as f64 * 0.2)) as i16;
                let y_travel = (progress * 12.0) as i16;
                (start_cx - x_travel, start_cy - y_travel + stagger)
            }
            FloatDir::Right => {
                let x_travel = (progress * (area.width as f64 * 0.2)) as i16;
                let y_travel = (progress * 12.0) as i16;
                (start_cx + x_travel, start_cy - y_travel + stagger)
            }
        };

        let half_text = ft.text.len() as i16 / 2;
        let x = (fx - half_text).max(area.x as i16);
        let y = fy.max(area.y as i16);

        if x < 0 || y < 0 {
            continue;
        }
        let x = x as u16;
        let y = y as u16;

        if y >= area.y + area.height || x >= area.x + area.width {
            continue;
        }

        let max_w = (area.x + area.width).saturating_sub(x);
        let w = (ft.text.len() as u16).min(max_w);
        if w == 0 {
            continue;
        }
        let text_area = Rect::new(x, y, w, 1);

        // Fade effect: bold -> normal -> dim
        let style = if ft.ticks_remaining < 10 {
            Style::default().fg(Color::DarkGray)
        } else if ft.ticks_remaining < 20 {
            Style::default().fg(ft.color)
        } else {
            Style::default()
                .fg(ft.color)
                .add_modifier(Modifier::BOLD)
        };

        let span = Paragraph::new(Span::styled(&ft.text, style));
        frame.render_widget(span, text_area);
    }
}
