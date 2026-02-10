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

    // Split: items list + total at bottom
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),   // item list
            Constraint::Length(2), // total + hint
        ])
        .split(area);

    let mut lines = Vec::new();

    let rarities = [
        Rarity::Mythic,
        Rarity::Legendary,
        Rarity::Epic,
        Rarity::Rare,
        Rarity::Uncommon,
        Rarity::Common,
    ];

    // Build display list with proper scrolling
    let mut display_items: Vec<(usize, &crate::game::item::ItemInstance)> = Vec::new();
    let mut selected_line: u16 = 0;

    // Show items organized by rarity (compact view)
    for rarity in &rarities {
        let items_of_rarity: Vec<(usize, &crate::game::item::ItemInstance)> = inv.items
            .iter()
            .enumerate()
            .filter(|(_, i)| i.rarity == *rarity)
            .collect();

        if items_of_rarity.is_empty() {
            continue;
        }

        // Section header with rarity color bar
        let total_count: u32 = items_of_rarity.iter().map(|(_, i)| i.count).sum();
        let bar_char = "\u{2588}"; // full block
        lines.push(Line::from(vec![
            Span::styled(
                bar_char.repeat(2),
                Style::default().fg(rarity.color()),
            ),
            Span::raw(" "),
            rarity_span(*rarity),
            Span::styled(
                format!(" \u{00d7}{}", total_count),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));

        // Show individual items (compact: one line per item)
        for (idx, item) in items_of_rarity {
            let display_idx = display_items.len();
            display_items.push((idx, item));

            let is_selected = display_idx == app.tab_scroll;

            if is_selected {
                selected_line = lines.len() as u16;
            }

            let marker = if is_selected { "\u{25b6}" } else { " " };

            let name_style = if is_selected {
                Style::default()
                    .fg(rarity.color())
                    .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
            } else {
                Style::default().fg(rarity.color())
            };

            let crit_marker = if item.is_crit { "\u{2605}" } else { "" };

            // Compact format: [marker] name [×count] [crit] | GP | XP
            let gp_str = if item.gp_value >= 1_000_000_000_000_000_000 {
                format!("{:.2}Qi", item.gp_value as f64 / 1_000_000_000_000_000_000.0)
            } else if item.gp_value >= 1_000_000_000_000_000 {
                format!("{:.2}Qa", item.gp_value as f64 / 1_000_000_000_000_000.0)
            } else if item.gp_value >= 1_000_000_000_000 {
                format!("{:.2}T", item.gp_value as f64 / 1_000_000_000_000.0)
            } else if item.gp_value >= 1_000_000_000 {
                format!("{:.2}B", item.gp_value as f64 / 1_000_000_000.0)
            } else if item.gp_value >= 1_000_000 {
                format!("{:.2}M", item.gp_value as f64 / 1_000_000.0)
            } else if item.gp_value >= 1_000 {
                format!("{:.1}K", item.gp_value as f64 / 1_000.0)
            } else {
                item.gp_value.to_string()
            };

            let xp_str = if item.xp_value >= 1_000_000_000_000_000_000 {
                format!("{:.2}Qi", item.xp_value as f64 / 1_000_000_000_000_000_000.0)
            } else if item.xp_value >= 1_000_000_000_000_000 {
                format!("{:.2}Qa", item.xp_value as f64 / 1_000_000_000_000_000.0)
            } else if item.xp_value >= 1_000_000_000_000 {
                format!("{:.2}T", item.xp_value as f64 / 1_000_000_000_000.0)
            } else if item.xp_value >= 1_000_000_000 {
                format!("{:.2}B", item.xp_value as f64 / 1_000_000_000.0)
            } else if item.xp_value >= 1_000_000 {
                format!("{:.2}M", item.xp_value as f64 / 1_000_000.0)
            } else if item.xp_value >= 1_000 {
                format!("{:.1}K", item.xp_value as f64 / 1_000.0)
            } else {
                item.xp_value.to_string()
            };

            let count_str = if item.count > 1 {
                format!(" \u{00d7}{}", item.count)
            } else {
                String::new()
            };

            let mut line_spans = vec![
                Span::styled(marker, Style::default().fg(Color::Yellow)),
                Span::raw(" "),
                Span::styled(item.name.clone(), name_style),
                Span::styled(count_str, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                Span::styled(crit_marker, Style::default().fg(Color::Yellow)),
                Span::raw("  "),
                Span::styled(
                    format!("{}GP", gp_str),
                    Style::default().fg(Color::Rgb(100, 100, 60)),
                ),
                Span::raw(" "),
                Span::styled(
                    format!("{}XP", xp_str),
                    Style::default().fg(Color::Cyan),
                ),
            ];

            if is_selected && app.state.skill_tree.has_skill("transmute_basics") {
                line_spans.push(Span::raw("  "));
                line_spans.push(Span::styled(
                    "[S]Sell",
                    Style::default().fg(Color::DarkGray),
                ));
            }

            lines.push(Line::from(line_spans));
        }

        lines.push(Line::from(""));
    }

    // Calculate scroll offset for smooth scrolling
    let viewport_height = sections[0].height.saturating_sub(1);
    let scroll_offset = if selected_line >= viewport_height {
        selected_line.saturating_sub(viewport_height) + 1
    } else {
        0
    };

    let paragraph = Paragraph::new(lines).scroll((scroll_offset, 0));
    frame.render_widget(paragraph, sections[0]);

    // Total at bottom with sell hints
    let can_sell = app.state.skill_tree.has_skill("transmute_basics");

    let total_lines = if can_sell {
        vec![
            Line::from(vec![
                Span::styled(
                    format!(" Total: {} items", inv.count()),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::raw("  "),
                Span::styled(
                    "[\u{2191}\u{2193}] Navigate  [S] Sell Selected  [A] Sell All",
                    Style::default().fg(Color::DarkGray),
                ),
            ]),
        ]
    } else {
        vec![
            Line::from(vec![
                Span::styled(
                    format!(" Total: {} items", inv.count()),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::raw("  "),
                Span::styled(
                    "[\u{2191}\u{2193}] Navigate  (Learn Transmute Basics to sell items)",
                    Style::default().fg(Color::DarkGray),
                ),
            ]),
        ]
    };

    frame.render_widget(Paragraph::new(total_lines), sections[1]);
}

