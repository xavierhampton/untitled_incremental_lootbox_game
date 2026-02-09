use ratatui::prelude::*;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::app::App;
use crate::data::skills::{SkillBranch, all_skills, get_skill};

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let skills = all_skills();
    let tree = &app.state.skill_tree;

    // Clamp scroll to valid range
    let tab_scroll = app.tab_scroll.min(skills.len().saturating_sub(1));

    let mut lines = Vec::new();
    let mut selected_line: u16 = 0;

    // Skill points header
    lines.push(Line::from(vec![
        Span::styled(
            format!(" Skill Points: {} ", tree.skill_points),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("  ({} learned)", tree.total_learned()),
            Style::default().fg(Color::DarkGray),
        ),
    ]));
    lines.push(Line::from(""));

    let mut current_branch: Option<SkillBranch> = None;
    let mut skill_index = 0usize;

    for skill in skills {
        // Branch header
        if current_branch != Some(skill.branch) {
            if current_branch.is_some() {
                lines.push(Line::from(""));
            }
            let branch_color = skill.branch.color();
            let header = format!(" {} ", skill.branch.label());
            let pad_len = 34usize.saturating_sub(header.len());
            let pad = "\u{2500}".repeat(pad_len);
            lines.push(Line::from(vec![
                Span::styled("\u{2500}\u{2500}", Style::default().fg(branch_color)),
                Span::styled(
                    header,
                    Style::default()
                        .fg(branch_color)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(pad, Style::default().fg(branch_color)),
            ]));
            current_branch = Some(skill.branch);
        }

        let is_learned = tree.has_skill(skill.id);
        let can_learn = tree.can_learn(skill.id);
        let prereqs_met = skill.prerequisites.iter().all(|p| tree.has_skill(p));
        let is_selected = skill_index == tab_scroll;

        if is_selected {
            selected_line = lines.len() as u16;
        }

        // Status marker
        let (marker, marker_color) = if is_learned {
            ("[*]", Color::Green)
        } else if can_learn {
            ("[ ]", Color::Yellow)
        } else {
            ("[x]", Color::DarkGray)
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

        let selector = if is_selected { "\u{25b6}" } else { " " };

        let cost_str = if skill.cost > 1 {
            format!(" ({}pt)", skill.cost)
        } else {
            " (1pt)".to_string()
        };

        let cost_color = if is_learned {
            Color::DarkGray
        } else if tree.skill_points >= skill.cost {
            Color::Cyan
        } else {
            Color::Red
        };

        lines.push(Line::from(vec![
            Span::styled(format!("{} ", selector), Style::default().fg(Color::Yellow)),
            Span::styled(format!("{} ", marker), Style::default().fg(marker_color)),
            Span::styled(format!("{:<18}", skill.name), name_style),
            Span::styled(cost_str, Style::default().fg(cost_color)),
            Span::raw(" "),
            Span::styled(
                skill.description,
                if is_learned {
                    Style::default().fg(Color::DarkGray)
                } else {
                    Style::default().fg(Color::Gray)
                },
            ),
        ]));

        // Show prerequisites if not met and selected
        if is_selected && !prereqs_met && !is_learned {
            let prereq_names: Vec<&str> = skill
                .prerequisites
                .iter()
                .filter(|p| !tree.has_skill(p))
                .filter_map(|p| get_skill(p).map(|s| s.name))
                .collect();
            if !prereq_names.is_empty() {
                lines.push(Line::from(vec![
                    Span::raw("       "),
                    Span::styled(
                        format!("\u{2514} Requires: {}", prereq_names.join(", ")),
                        Style::default()
                            .fg(Color::Red)
                            .add_modifier(Modifier::ITALIC),
                    ),
                ]));
            }
        }

        skill_index += 1;
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        " [B] Learn selected  [\u{2191}\u{2193}] Navigate",
        Style::default().fg(Color::DarkGray),
    )));

    // Scroll just enough to keep the selected line visible (smooth, 1-line-at-a-time)
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
