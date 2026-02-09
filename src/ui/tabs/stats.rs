use ratatui::prelude::*;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let stats = &app.state.stats;
    let player = &app.state.player;

    let mut lines = vec![
        Line::from(vec![
            Span::styled(
                " \u{2500}\u{2500} ",
                Style::default().fg(Color::Yellow),
            ),
            Span::styled(
                "Lifetime Statistics",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}",
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(""),
    ];

    let stat_entries = [
        ("Chests Opened", format_num(stats.chests_opened), Color::White),
        ("Items Found", format_num(stats.items_found), Color::White),
        ("Total GP Earned", format_num(stats.total_gp_earned), Color::Yellow),
        ("Total XP Earned", format_num(stats.total_xp_earned), Color::Cyan),
        ("Mythics", format_num(stats.mythics_found), Color::Rgb(255, 50, 50)),
        ("Legendaries", format_num(stats.legendaries_found), Color::Yellow),
        ("Epics", format_num(stats.epics_found), Color::Magenta),
        ("Rares", format_num(stats.rares_found), Color::Blue),
        ("Critical Hits", format_num(stats.crits_rolled), Color::Red),
        ("Best Single Drop", format!("{} GP", format_num(stats.highest_single_gp)), Color::Yellow),
    ];

    for (label, value, value_color) in &stat_entries {
        lines.push(Line::from(vec![
            Span::styled(
                format!("  {:<20}", label),
                Style::default().fg(Color::Gray),
            ),
            Span::styled(
                value.clone(),
                Style::default()
                    .fg(*value_color)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
    }

    // Rebirth info
    if app.state.rebirth.rebirth_count > 0 || app.state.rebirth.total_essence_earned > 0 {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled(
                " \u{2500}\u{2500} ",
                Style::default().fg(Color::Rgb(150, 100, 255)),
            ),
            Span::styled(
                "Rebirth",
                Style::default()
                    .fg(Color::Rgb(150, 100, 255))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}",
                Style::default().fg(Color::Rgb(150, 100, 255)),
            ),
        ]));
        lines.push(Line::from(""));

        let rebirth_entries = [
            ("Rebirth Count", format_num(app.state.rebirth.rebirth_count as u64), Color::Rgb(150, 100, 255)),
            ("Total Essence", format_num(app.state.rebirth.total_essence_earned), Color::Rgb(200, 150, 255)),
            ("Highest Level", format_num(app.state.rebirth.highest_level_ever as u64), Color::Cyan),
            ("Rebirth Skills", format_num(app.state.rebirth.rebirth_skills.len() as u64), Color::Green),
        ];

        for (label, value, value_color) in &rebirth_entries {
            lines.push(Line::from(vec![
                Span::styled(
                    format!("  {:<20}", label),
                    Style::default().fg(Color::Gray),
                ),
                Span::styled(
                    value.clone(),
                    Style::default()
                        .fg(*value_color)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled(
            " \u{2500}\u{2500} ",
            Style::default().fg(Color::Cyan),
        ),
        Span::styled(
            "Current Stats",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            " \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}",
            Style::default().fg(Color::Cyan),
        ),
    ]));
    lines.push(Line::from(""));

    // Stats with visual bars
    let player_stats: Vec<(&str, String, f64, f64, Color)> = vec![
        ("Level", player.level.to_string(), player.level as f64, 50.0, Color::White),
        ("Luck", format!("{:.1}", player.luck), player.luck, 50.0, Color::Green),
        ("Speed", format!("{:.2}x", player.speed), player.speed, 5.0, Color::Cyan),
        ("GP Mult", format!("{:.2}x", player.gp_multiplier), player.gp_multiplier, 5.0, Color::Yellow),
        ("XP Mult", format!("{:.2}x", player.xp_multiplier), player.xp_multiplier, 5.0, Color::Cyan),
        ("Crit", format!("{:.1}%", player.crit_chance * 100.0), player.crit_chance * 100.0, 75.0, Color::Red),
    ];

    for (label, value, current, max_val, color) in &player_stats {
        let bar_width: usize = 10;
        let filled = ((current / max_val) * bar_width as f64).min(bar_width as f64) as usize;
        let empty = bar_width.saturating_sub(filled);
        let bar = format!("{}{}", "\u{2588}".repeat(filled), "\u{2591}".repeat(empty));

        lines.push(Line::from(vec![
            Span::styled(
                format!("  {:<12}", label),
                Style::default().fg(Color::Gray),
            ),
            Span::styled(
                bar,
                Style::default().fg(*color),
            ),
            Span::raw(" "),
            Span::styled(
                value.clone(),
                Style::default()
                    .fg(*color)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        " [\u{2191}\u{2193}] Scroll",
        Style::default().fg(Color::DarkGray),
    )));

    let scroll = app.tab_scroll as u16;
    let paragraph = Paragraph::new(lines).scroll((scroll, 0));
    frame.render_widget(paragraph, area);
}

fn format_num(n: u64) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}
