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

    let mut lines = Vec::new();

    for (i, relic_id) in owned.iter().enumerate() {
        let is_selected = i == app.tab_scroll;
        let is_equipped = app.state.relics.is_equipped(relic_id);

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

            // Always show rarity tag and effect
            lines.push(Line::from(vec![
                Span::raw("     "),
                Span::styled(
                    format!("[{}]", relic_def.rarity.label()),
                    Style::default().fg(relic_def.rarity.color()),
                ),
                Span::raw(" "),
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

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, sections[0]);

    // Footer with equip info
    let equipped_count = app.state.relics.equipped.len();
    let max = crate::game::relic::RelicState::MAX_EQUIPPED;
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
            " [E] Equip/Unequip  [\u{2191}\u{2193}] Navigate",
            Style::default().fg(Color::DarkGray),
        )),
    ];
    frame.render_widget(Paragraph::new(footer_lines), sections[1]);
}
