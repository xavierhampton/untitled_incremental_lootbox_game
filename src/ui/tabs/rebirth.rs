use ratatui::prelude::*;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::app::App;
use crate::data::rebirth_skills::all_rebirth_skills;

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let rb = &app.state.rebirth;
    let player = &app.state.player;

    let mut lines = Vec::new();

    // === Rebirth Status Section ===
    lines.push(Line::from(vec![
        Span::styled(
            "\u{2500}\u{2500} ",
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

    // Stats
    lines.push(Line::from(vec![
        Span::styled("  Rebirth Count:  ", Style::default().fg(Color::Gray)),
        Span::styled(
            format!("{}", rb.rebirth_count),
            Style::default()
                .fg(Color::Rgb(150, 100, 255))
                .add_modifier(Modifier::BOLD),
        ),
    ]));
    lines.push(Line::from(vec![
        Span::styled("  Essence:        ", Style::default().fg(Color::Gray)),
        Span::styled(
            format!("{}", rb.essence),
            Style::default()
                .fg(Color::Rgb(200, 150, 255))
                .add_modifier(Modifier::BOLD),
        ),
    ]));
    lines.push(Line::from(vec![
        Span::styled("  Total Earned:   ", Style::default().fg(Color::Gray)),
        Span::styled(
            format!("{}", rb.total_essence_earned),
            Style::default().fg(Color::Rgb(200, 150, 255)),
        ),
    ]));
    lines.push(Line::from(vec![
        Span::styled("  Current Level:  ", Style::default().fg(Color::Gray)),
        Span::styled(
            format!("{}", player.level),
            Style::default().fg(Color::Cyan),
        ),
    ]));

    let min_level = rb.min_level_for_rebirth();
    let can_rebirth = rb.can_rebirth(player.level);
    lines.push(Line::from(vec![
        Span::styled("  Min Level:      ", Style::default().fg(Color::Gray)),
        Span::styled(
            format!("{}", min_level),
            if can_rebirth {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::Red)
            },
        ),
    ]));

    // Estimated essence reward
    let est_essence = rb.calculate_essence_reward(player.level, rb.gp_earned_this_run);
    lines.push(Line::from(vec![
        Span::styled("  Est. Reward:    ", Style::default().fg(Color::Gray)),
        Span::styled(
            format!("{} Essence", est_essence),
            Style::default()
                .fg(Color::Rgb(200, 150, 255))
                .add_modifier(Modifier::BOLD),
        ),
    ]));

    lines.push(Line::from(""));

    // Rebirth button
    if can_rebirth {
        if app.rebirth_confirm {
            lines.push(Line::from(Span::styled(
                "  [R] CONFIRM REBIRTH - Press R again!",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )));
        } else {
            lines.push(Line::from(Span::styled(
                "  [R] Rebirth",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )));
        }
    } else {
        lines.push(Line::from(Span::styled(
            format!("  [R] Rebirth (need level {})", min_level),
            Style::default().fg(Color::DarkGray),
        )));
    }

    lines.push(Line::from(""));

    // === Rebirth Skill Tree Section ===
    lines.push(Line::from(vec![
        Span::styled(
            "\u{2500}\u{2500} ",
            Style::default().fg(Color::Rgb(200, 150, 255)),
        ),
        Span::styled(
            "Rebirth Skills",
            Style::default()
                .fg(Color::Rgb(200, 150, 255))
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            " \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}",
            Style::default().fg(Color::Rgb(200, 150, 255)),
        ),
    ]));
    lines.push(Line::from(""));

    let skills = all_rebirth_skills();
    let mut current_tier: Option<u32> = None;
    let tab_scroll = app.tab_scroll.min(skills.len().saturating_sub(1));
    let mut selected_line: u16 = 0;

    for (i, skill) in skills.iter().enumerate() {
        // Tier header
        if current_tier != Some(skill.tier) {
            if current_tier.is_some() {
                lines.push(Line::from(""));
            }
            let tier_label = match skill.tier {
                1 => "Tier 1 \u{2014} Foundations",
                2 => "Tier 2 \u{2014} Mastery",
                3 => "Tier 3 \u{2014} Transcendence",
                _ => "Unknown Tier",
            };
            let tier_color = match skill.tier {
                1 => Color::Rgb(120, 200, 120),
                2 => Color::Rgb(100, 150, 255),
                3 => Color::Rgb(255, 200, 50),
                _ => Color::White,
            };
            lines.push(Line::from(Span::styled(
                format!("  {}", tier_label),
                Style::default()
                    .fg(tier_color)
                    .add_modifier(Modifier::BOLD),
            )));
            current_tier = Some(skill.tier);
        }

        let is_learned = rb.has_rebirth_skill(skill.id);
        let can_learn = rb.can_learn_rebirth_skill(skill.id);
        let is_selected = i == tab_scroll;
        if is_selected {
            selected_line = lines.len() as u16;
        }

        let marker = if is_learned {
            "[*]"
        } else if can_learn {
            "[ ]"
        } else {
            "[x]"
        };

        let marker_color = if is_learned {
            Color::Green
        } else if can_learn {
            Color::Yellow
        } else {
            Color::DarkGray
        };

        let name_style = if is_selected {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else if is_learned {
            Style::default().fg(Color::Green)
        } else if can_learn {
            Style::default().fg(Color::White)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let cost_str = if is_learned {
            "Learned".to_string()
        } else {
            format!("{} Ess", skill.essence_cost)
        };

        let cost_color = if is_learned {
            Color::Green
        } else if rb.essence >= skill.essence_cost {
            Color::Rgb(200, 150, 255)
        } else {
            Color::Red
        };

        lines.push(Line::from(vec![
            Span::styled(format!("  {} ", marker), Style::default().fg(marker_color)),
            Span::styled(format!("{:<22}", skill.name), name_style),
            Span::styled(cost_str, Style::default().fg(cost_color)),
        ]));

        if is_selected {
            lines.push(Line::from(vec![
                Span::raw("      "),
                Span::styled(
                    skill.description,
                    Style::default()
                        .fg(Color::Gray)
                        .add_modifier(Modifier::ITALIC),
                ),
            ]));

            // Show prerequisites if not met
            if !is_learned && !can_learn && rb.essence >= skill.essence_cost {
                let missing: Vec<&str> = skill
                    .prerequisites
                    .iter()
                    .filter(|pre| !rb.has_rebirth_skill(pre))
                    .copied()
                    .collect();
                if !missing.is_empty() {
                    lines.push(Line::from(vec![
                        Span::raw("      "),
                        Span::styled(
                            format!("Requires: {}", missing.join(", ")),
                            Style::default()
                                .fg(Color::Red)
                                .add_modifier(Modifier::ITALIC),
                        ),
                    ]));
                }
            }
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        " [E] Buy skill  [R] Rebirth  [\u{2191}\u{2193}] Navigate",
        Style::default().fg(Color::DarkGray),
    )));

    let visible_height = area.height;
    let margin = 2u16;
    let scroll_y = if selected_line + margin >= visible_height {
        (selected_line + margin + 1).saturating_sub(visible_height)
    } else {
        0
    };
    let paragraph = Paragraph::new(lines).scroll((scroll_y, 0));
    frame.render_widget(paragraph, area);
}
