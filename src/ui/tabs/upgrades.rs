use ratatui::prelude::*;
use ratatui::widgets::{Gauge, Paragraph};
use ratatui::Frame;

use crate::app::App;
use crate::data::upgrades::{UpgradeCategory, all_upgrades};

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let upgrades = all_upgrades();

    // Split: content area + GP bar at bottom
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(2)])
        .split(area);

    let mut lines = Vec::new();
    let mut selected_line: u16 = 0;
    let mut current_category: Option<UpgradeCategory> = None;
    let tab_scroll = app.tab_scroll.min(upgrades.len().saturating_sub(1));

    for (i, upg) in upgrades.iter().enumerate() {
        // Category header
        if current_category != Some(upg.category) {
            if current_category.is_some() {
                lines.push(Line::from(""));
            }
            let cat_color = match upg.category {
                UpgradeCategory::Speed => Color::Cyan,
                UpgradeCategory::Luck => Color::Green,
                UpgradeCategory::Wealth => Color::Yellow,
                UpgradeCategory::Mastery => Color::Red,
                UpgradeCategory::Discovery => Color::Rgb(100, 200, 100),
                UpgradeCategory::Unlock => Color::Magenta,
            };
            let header = format!(" {} ", upg.category.label());
            let pad_len = 30usize.saturating_sub(header.len());
            let pad = "\u{2500}".repeat(pad_len);
            lines.push(Line::from(vec![
                Span::styled(
                    "\u{2500}\u{2500}",
                    Style::default().fg(cat_color),
                ),
                Span::styled(
                    header,
                    Style::default()
                        .fg(cat_color)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(pad, Style::default().fg(cat_color)),
            ]));
            current_category = Some(upg.category);
        }

        let level = app.state.upgrades.get_level(upg.id);
        let maxed = level >= upg.max_level;
        let cost = if maxed {
            "MAX".to_string()
        } else {
            format!("{} GP", upg.cost_at_level(level))
        };

        let is_selected = i == tab_scroll;
        if is_selected {
            selected_line = lines.len() as u16;
        }
        let can_afford = !maxed && app.state.player.gp >= upg.cost_at_level(level);

        let marker = if is_selected { "\u{25b6}" } else { " " };
        let name_style = if is_selected {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else if maxed {
            Style::default().fg(Color::DarkGray)
        } else if can_afford {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::White)
        };

        // Level progress visualization: [####----] 4/10
        let filled = level as usize;
        let empty = (upg.max_level as usize).saturating_sub(filled);
        let bar = format!(
            "[{}{}]",
            "#".repeat(filled.min(20)),
            "-".repeat(empty.min(20))
        );

        lines.push(Line::from(vec![
            Span::styled(format!("{} ", marker), Style::default().fg(Color::Yellow)),
            Span::styled(format!("{:<18}", upg.name), name_style),
            Span::styled(
                bar,
                if maxed {
                    Style::default().fg(Color::DarkGray)
                } else {
                    Style::default().fg(Color::Rgb(80, 80, 100))
                },
            ),
            Span::raw(" "),
            Span::styled(
                cost,
                if maxed {
                    Style::default().fg(Color::DarkGray)
                } else if can_afford {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::Red)
                },
            ),
        ]));

        if is_selected {
            lines.push(Line::from(vec![
                Span::raw("   "),
                Span::styled(
                    upg.description,
                    Style::default().fg(Color::Gray).add_modifier(Modifier::ITALIC),
                ),
                Span::raw("  "),
                Span::styled(
                    format!("Lv.{}/{}", level, upg.max_level),
                    Style::default().fg(Color::Cyan),
                ),
            ]));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        " [B] Buy selected  [\u{2191}\u{2193}] Navigate",
        Style::default().fg(Color::DarkGray),
    )));

    let visible_height = sections[0].height;
    let margin = 2u16;
    let scroll_y = if selected_line + margin >= visible_height {
        (selected_line + margin + 1).saturating_sub(visible_height)
    } else {
        0
    };
    let paragraph = Paragraph::new(lines).scroll((scroll_y, 0));
    frame.render_widget(paragraph, sections[0]);

    // GP bar at bottom
    let gp = app.state.player.gp;
    let gp_label = format!(" GP: {} ", format_number(gp));
    let gauge = Gauge::default()
        .gauge_style(Style::default().fg(Color::Yellow).bg(Color::Rgb(40, 40, 20)))
        .ratio(1.0)
        .label(gp_label);
    frame.render_widget(gauge, sections[1]);
}

fn format_number(n: u64) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}
