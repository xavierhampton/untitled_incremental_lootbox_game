use ratatui::prelude::*;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::app::App;
use crate::game::item::Rarity;
use crate::ui::widgets::rarity_label::rarity_span;

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let inv = &app.state.inventory;

    if inv.items.is_empty() {
        let lines = vec![
            Line::from(""),
            Line::from(""),
            Line::from(Span::styled(
                "    No items yet.",
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "    Press [Space] to open a chest!",
                Style::default().fg(Color::Gray),
            )),
            Line::from(""),
            Line::from(""),
            Line::from(Span::styled(
                "    Loot will appear here as you",
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(Span::styled(
                "    collect items from chests.",
                Style::default().fg(Color::DarkGray),
            )),
        ];
        let msg = Paragraph::new(lines);
        frame.render_widget(msg, area);
        return;
    }

    // Split: rarity distribution bar at top, items in middle, total at bottom
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // distribution bar
            Constraint::Min(1),   // item list
            Constraint::Length(1), // total + hint
        ])
        .split(area);

    // Rarity distribution bar
    draw_rarity_bar(frame, app, sections[0]);

    let mut lines = Vec::new();

    let rarities = [
        Rarity::Legendary,
        Rarity::Epic,
        Rarity::Rare,
        Rarity::Uncommon,
        Rarity::Common,
    ];

    for rarity in &rarities {
        let items_of_rarity: Vec<_> = inv.items.iter().filter(|i| i.rarity == *rarity).collect();
        if items_of_rarity.is_empty() {
            continue;
        }

        // Section header with rarity color bar
        let count = items_of_rarity.len();
        let bar_char = "\u{2588}"; // full block
        lines.push(Line::from(vec![
            Span::styled(
                bar_char.repeat(2),
                Style::default().fg(rarity.color()),
            ),
            Span::raw(" "),
            rarity_span(*rarity),
            Span::styled(
                format!(" \u{00d7}{}", count),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));

        // Count duplicates
        let mut name_counts: Vec<(String, usize, u64)> = Vec::new();
        for item in &items_of_rarity {
            if let Some(entry) = name_counts.iter_mut().find(|(n, _, _)| *n == item.name) {
                entry.1 += 1;
                entry.2 += item.gp_value;
            } else {
                name_counts.push((item.name.clone(), 1, item.gp_value));
            }
        }

        for (name, count, total_gp) in &name_counts {
            let count_str = if *count > 1 {
                format!(" \u{00d7}{}", count)
            } else {
                String::new()
            };
            lines.push(Line::from(vec![
                Span::styled("  \u{2022} ", Style::default().fg(rarity.color())),
                Span::styled(
                    name.clone(),
                    Style::default().fg(rarity.color()),
                ),
                Span::styled(count_str, Style::default().fg(Color::DarkGray)),
                Span::styled(
                    format!("  ({} GP)", total_gp),
                    Style::default().fg(Color::Rgb(100, 100, 60)),
                ),
            ]));
        }

        lines.push(Line::from(""));
    }

    let scroll = app.tab_scroll as u16;
    let paragraph = Paragraph::new(lines).scroll((scroll, 0));
    frame.render_widget(paragraph, sections[1]);

    // Total at bottom
    let total_line = Line::from(vec![
        Span::styled(
            format!(" Total: {} items", inv.count()),
            Style::default().fg(Color::DarkGray),
        ),
        Span::raw("  "),
        Span::styled(
            "[\u{2191}\u{2193}] Scroll",
            Style::default().fg(Color::DarkGray),
        ),
    ]);
    frame.render_widget(Paragraph::new(total_line), sections[2]);
}

fn draw_rarity_bar(frame: &mut Frame, app: &App, area: Rect) {
    let inv = &app.state.inventory;
    let total = inv.count() as f64;
    if total == 0.0 {
        return;
    }

    let rarities = [
        Rarity::Common,
        Rarity::Uncommon,
        Rarity::Rare,
        Rarity::Epic,
        Rarity::Legendary,
    ];

    let bar_width = area.width.saturating_sub(2) as f64;

    // First line: colored bar
    let mut bar_spans = Vec::new();
    bar_spans.push(Span::raw(" "));
    for rarity in &rarities {
        let count = inv.items.iter().filter(|i| i.rarity == *rarity).count() as f64;
        let width = ((count / total) * bar_width).round() as usize;
        if width > 0 {
            bar_spans.push(Span::styled(
                "\u{2588}".repeat(width),
                Style::default().fg(rarity.color()),
            ));
        }
    }

    // Second line: legend
    let mut legend_spans = Vec::new();
    legend_spans.push(Span::raw(" "));
    for rarity in &rarities {
        let count = inv.items.iter().filter(|i| i.rarity == *rarity).count();
        if count > 0 {
            let pct = (count as f64 / total * 100.0) as u32;
            legend_spans.push(Span::styled(
                "\u{25cf}",
                Style::default().fg(rarity.color()),
            ));
            legend_spans.push(Span::styled(
                format!("{}% ", pct),
                Style::default().fg(Color::DarkGray),
            ));
        }
    }

    let lines = vec![
        Line::from(bar_spans),
        Line::from(""),
        Line::from(legend_spans),
    ];

    frame.render_widget(Paragraph::new(lines), area);
}
