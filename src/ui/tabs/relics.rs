use ratatui::prelude::*;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::app::App;
use crate::data::relics::get_relic;

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let owned = &app.state.relics.owned;

    if owned.is_empty() {
        let lines = vec![
            Line::from(""),
            Line::from(Span::styled(
                "  \u{2728} No relics discovered yet",
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "  Relics are powerful artifacts that",
                Style::default().fg(Color::Rgb(80, 80, 80)),
            )),
            Line::from(Span::styled(
                "  boost your stats when equipped.",
                Style::default().fg(Color::Rgb(80, 80, 80)),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "  \u{2022} Drop from Gold+ chests",
                Style::default().fg(Color::Rgb(100, 100, 60)),
            )),
            Line::from(Span::styled(
                "  \u{2022} Higher rarity = better chance",
                Style::default().fg(Color::Rgb(100, 100, 60)),
            )),
            Line::from(Span::styled(
                "  \u{2022} Equip up to 5 at once",
                Style::default().fg(Color::Rgb(100, 100, 60)),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "  Keep opening chests!",
                Style::default().fg(Color::Gray),
            )),
        ];
        let msg = Paragraph::new(lines);
        frame.render_widget(msg, area);
        return;
    }

    // Split: relic list + footer
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(area);

    // Organize relics by rarity
    use crate::game::item::Rarity;
    let rarities_order = [
        Rarity::Mythic,
        Rarity::Legendary,
        Rarity::Epic,
        Rarity::Rare,
        Rarity::Uncommon,
    ];

    let mut organized_relics: Vec<(usize, String, Rarity)> = Vec::new();
    for (original_idx, relic_id) in owned.iter().enumerate() {
        if let Some(relic_def) = get_relic(relic_id) {
            organized_relics.push((original_idx, relic_id.clone(), relic_def.rarity));
        }
    }

    // Sort by rarity tier (Mythic first)
    organized_relics.sort_by_key(|(_, _, rarity)| {
        rarities_order.iter().position(|r| r == rarity).unwrap_or(99)
    });

    let mut lines = Vec::new();
    let mut current_rarity: Option<Rarity> = None;

    // Build mapping from display order to original index for selection
    // Also track which line each relic starts on for scrolling
    let mut display_idx_to_original: Vec<usize> = Vec::new();
    let mut selected_line: u16 = 0;

    for (original_idx, relic_id, rarity) in &organized_relics {
        // Add rarity header if this is a new rarity section
        if current_rarity != Some(*rarity) {
            if current_rarity.is_some() {
                lines.push(Line::from(""));
            }
            let bar_char = "\u{2588}"; // full block
            lines.push(Line::from(vec![
                Span::styled(
                    bar_char.repeat(2),
                    Style::default().fg(rarity.color()),
                ),
                Span::raw(" "),
                Span::styled(
                    rarity.label(),
                    Style::default()
                        .fg(rarity.color())
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
            current_rarity = Some(*rarity);
        }

        // Track mapping for cursor selection
        let display_idx = display_idx_to_original.len();
        display_idx_to_original.push(*original_idx);

        let is_selected = display_idx == app.tab_scroll;
        let is_equipped = app.state.relics.is_equipped(relic_id);

        // Track which line the selected relic is on
        if is_selected {
            selected_line = lines.len() as u16;
        }

        if let Some(relic_def) = get_relic(relic_id) {
            let marker = if is_selected { "\u{25b6}" } else { " " };
            let equipped_icon = if is_equipped { "\u{2605}" } else { "\u{2606}" };

            let name_style = Style::default()
                .fg(relic_def.rarity.color())
                .add_modifier(if is_selected {
                    Modifier::BOLD | Modifier::UNDERLINED
                } else {
                    Modifier::BOLD
                });

            lines.push(Line::from(vec![
                Span::styled(
                    format!("{} ", marker),
                    Style::default().fg(Color::Yellow),
                ),
                Span::styled(
                    format!("{} ", equipped_icon),
                    if is_equipped {
                        Style::default().fg(Color::Yellow)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    },
                ),
                Span::styled(relic_def.name, name_style),
            ]));

            // Always show effect
            lines.push(Line::from(vec![
                Span::raw("     "),
                Span::styled(
                    relic_def.description,
                    Style::default().fg(Color::Gray),
                ),
            ]));

            if is_selected {
                let status = if is_equipped {
                    "Equipped \u{2014} Press [E] to unequip"
                } else {
                    "Not equipped \u{2014} Press [E] to equip"
                };
                lines.push(Line::from(Span::styled(
                    format!("     {}", status),
                    Style::default().fg(if is_equipped {
                        Color::Green
                    } else {
                        Color::DarkGray
                    }),
                )));
            }

            lines.push(Line::from(""));
        }
    }

    // Calculate scroll offset to keep selected relic visible (smooth scrolling)
    let viewport_height = sections[0].height.saturating_sub(1);
    let scroll_offset = if selected_line >= viewport_height {
        // Start scrolling when cursor reaches bottom
        selected_line.saturating_sub(viewport_height) + 1
    } else {
        0
    };

    let paragraph = Paragraph::new(lines).scroll((scroll_offset, 0));
    frame.render_widget(paragraph, sections[0]);

    // Footer with equip info
    let equipped_count = app.state.relics.equipped.len();
    let max = app.max_equipped_relics();
    let slot_bar: String = (0..max)
        .map(|i| if i < equipped_count { '\u{25c6}' } else { '\u{25c7}' })
        .collect();
    let footer_lines = vec![
        Line::from(vec![
            Span::styled(" Slots: ", Style::default().fg(Color::Gray)),
            Span::styled(
                slot_bar,
                Style::default().fg(Color::Yellow),
            ),
            Span::styled(
                format!(" {}/{}", equipped_count, max),
                Style::default().fg(Color::DarkGray),
            ),
        ]),
        Line::from(Span::styled(
            " [E] Equip/Unequip  [U] Unequip All  [\u{2191}\u{2193}] Navigate",
            Style::default().fg(Color::DarkGray),
        )),
    ];
    frame.render_widget(Paragraph::new(footer_lines), sections[1]);
}
