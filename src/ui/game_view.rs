use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Gauge, Paragraph};
use ratatui::Frame;

use crate::app::App;
use crate::game::chest::{ChestState, ChestType};
use super::chest_art::get_chest_art;
use super::widgets::rarity_label::rarity_span;

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.state.current_chest_type.color()))
        .title(format!(" {} Chest ", app.state.current_chest_type.name()));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Top: chest selector + art (flexible), Bottom: HUD pinned to bottom
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),  // chest selector
            Constraint::Min(9),    // chest art (fills available space)
            Constraint::Length(2),  // progress bar
            Constraint::Length(4),  // reveal area
            Constraint::Length(3),  // player status
            Constraint::Length(3),  // messages
        ])
        .split(inner);

    // Chest selector (top)
    draw_chest_selector(frame, app, sections[0]);

    // Chest art (centered in flexible middle area)
    draw_chest_art(frame, app, sections[1]);

    // HUD pinned to bottom
    draw_progress_bar(frame, app, sections[2]);
    draw_reveal(frame, app, sections[3]);
    draw_player_status(frame, app, sections[4]);
    draw_messages(frame, app, sections[5]);
}

fn draw_chest_selector(frame: &mut Frame, app: &App, area: Rect) {
    let line = Line::from(Span::styled(
        "Press [C] for chests",
        Style::default().fg(Color::DarkGray),
    ));
    frame.render_widget(Paragraph::new(line).alignment(Alignment::Center), area);
}

fn draw_chest_art(frame: &mut Frame, app: &App, area: Rect) {
    let art = get_chest_art(
        app.state.current_chest_type,
        app.state.chest_progress.state,
        app.state.chest_progress.ticks_elapsed,
    );

    let lines: Vec<Line> = art
        .lines
        .iter()
        .map(|l| {
            Line::from(Span::styled(
                *l,
                Style::default().fg(app.state.current_chest_type.color()),
            ))
        })
        .collect();

    // Vertically center the art within the available area
    let art_height = lines.len() as u16;
    let y_offset = area.height.saturating_sub(art_height) / 2;
    let centered_area = Rect::new(area.x, area.y + y_offset, area.width, art_height.min(area.height));

    let paragraph = Paragraph::new(lines).alignment(Alignment::Center);
    frame.render_widget(paragraph, centered_area);
}

fn draw_progress_bar(frame: &mut Frame, app: &App, area: Rect) {
    let progress = app.state.chest_progress.progress_fraction();
    let label = match app.state.chest_progress.state {
        ChestState::Idle => "Press [Space] to open".to_string(),
        ChestState::Opening => format!("Opening... {:.0}%", progress * 100.0),
        ChestState::Revealing => "Collect! [Space]".to_string(),
        ChestState::Complete => "Done!".to_string(),
    };

    let gauge = Gauge::default()
        .gauge_style(
            Style::default()
                .fg(app.state.current_chest_type.color())
                .bg(Color::DarkGray),
        )
        .ratio(progress)
        .label(label);
    frame.render_widget(gauge, area);
}

fn draw_reveal(frame: &mut Frame, app: &App, area: Rect) {
    if let Some(ref item) = app.state.chest_progress.last_item {
        let mut lines = vec![
            Line::from(vec![
                rarity_span(item.rarity),
                Span::styled(
                    format!(" {}", item.name),
                    Style::default()
                        .fg(item.rarity.color())
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(format!("  +{} GP  +{} XP", item.gp_value, item.xp_value)),
        ];
        if item.is_crit {
            lines.push(Line::from(Span::styled(
                "  * CRITICAL HIT! *",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )));
        }
        let paragraph = Paragraph::new(lines).alignment(Alignment::Center);
        frame.render_widget(paragraph, area);
    }
}

fn draw_player_status(frame: &mut Frame, app: &App, area: Rect) {
    let player = &app.state.player;
    let xp_progress = if player.xp_to_next > 0 {
        player.xp as f64 / player.xp_to_next as f64
    } else {
        0.0
    };

    let status_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(1), Constraint::Length(1)])
        .split(area);

    // Level + XP bar
    let level_line = Line::from(vec![
        Span::styled(
            format!("Lv.{}", player.level),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!(
            "  XP: {}/{}",
            player.xp, player.xp_to_next
        )),
    ]);
    frame.render_widget(Paragraph::new(level_line), status_layout[0]);

    // XP gauge
    let xp_gauge = Gauge::default()
        .gauge_style(Style::default().fg(Color::Cyan).bg(Color::DarkGray))
        .ratio(xp_progress.min(1.0))
        .label("");
    frame.render_widget(xp_gauge, status_layout[1]);

    // GP + Stats + Rebirth info
    let mut stats_spans = vec![
        Span::styled(
            format!("GP: {}", format_number(player.gp)),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
    ];
    if app.state.rebirth.rebirth_count > 0 || app.state.rebirth.essence > 0 {
        stats_spans.push(Span::styled(
            format!("  Ess: {}", app.state.rebirth.essence),
            Style::default().fg(Color::Rgb(200, 150, 255)),
        ));
    }
    stats_spans.push(Span::raw(format!(
        "  Lk: {:.0}  Spd: {:.1}x  Crt: {:.0}%",
        player.luck,
        player.speed,
        player.crit_chance * 100.0
    )));
    let stats_line = Line::from(stats_spans);
    frame.render_widget(Paragraph::new(stats_line), status_layout[2]);
}

fn draw_messages(frame: &mut Frame, app: &App, area: Rect) {
    let lines: Vec<Line> = app
        .message_log
        .iter()
        .rev()
        .take(area.height as usize)
        .map(|(msg, ticks)| {
            let color = if *ticks < 30 {
                Color::DarkGray
            } else {
                Color::White
            };
            Line::from(Span::styled(msg.as_str(), Style::default().fg(color)))
        })
        .collect();

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, area);
}

fn format_number(n: u64) -> String {
    if n >= 1_000_000_000_000_000_000 {
        format!("{:.2}Qi", n as f64 / 1_000_000_000_000_000_000.0)
    } else if n >= 1_000_000_000_000_000 {
        format!("{:.2}Qa", n as f64 / 1_000_000_000_000_000.0)
    } else if n >= 1_000_000_000_000 {
        format!("{:.2}T", n as f64 / 1_000_000_000_000.0)
    } else if n >= 1_000_000_000 {
        format!("{:.2}B", n as f64 / 1_000_000_000.0)
    } else if n >= 1_000_000 {
        format!("{:.2}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}
