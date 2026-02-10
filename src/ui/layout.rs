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
    let footer_text = " [Space] Open  [C] Chest Menu  [\u{2190}/\u{2192}] Tabs  [E] Buy/Learn/Equip  [Esc] Settings";
    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(footer, outer[1]);

    // Help overlay
    if app.show_help {
        draw_help_overlay(frame, size);
    }

    // Chest menu overlay
    if app.show_chest_menu {
        draw_chest_menu_overlay(frame, app, columns[0]);
    }

    // Settings overlay
    if app.show_settings {
        draw_settings_overlay(frame, app, size);
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
    let overlay_height = 18.min(area.height.saturating_sub(4));
    let x = (area.width.saturating_sub(overlay_width)) / 2;
    let y = (area.height.saturating_sub(overlay_height)) / 2;
    let overlay_area = Rect::new(x, y, overlay_width, overlay_height);

    // Clear background
    let clear = ratatui::widgets::Clear;
    frame.render_widget(clear, overlay_area);

    let help_text = vec![
        Line::from(Span::styled("Controls", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(" Space        Open / Collect / Pause*"),
        Line::from(" E            Buy / Learn / Equip"),
        Line::from(" Left/Right   Switch tabs"),
        Line::from(" C            Toggle chest menu"),
        Line::from(" 1-7          Select chest type"),
        Line::from(" Up/Down      Scroll list"),
        Line::from(" U            Unequip all relics"),
        Line::from(" S            Sell item (Alchemy)"),
        Line::from(" A            Sell all items (Alchemy)"),
        Line::from(" Q            Quit (auto-saves)"),
        Line::from(" ?            Toggle this help"),
        Line::from(""),
        Line::from(Span::styled("* Space pauses/resumes when auto opener is active", Style::default().fg(Color::DarkGray))),
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

fn draw_chest_menu_overlay(frame: &mut Frame, app: &App, area: Rect) {
    use crate::game::chest::ChestType;

    let overlay_width = 50.min(area.width.saturating_sub(4));
    let overlay_height = 20.min(area.height.saturating_sub(4));
    let x = (area.width.saturating_sub(overlay_width)) / 2;
    let y = area.y + 2; // Position near top of game view
    let overlay_area = Rect::new(x, y, overlay_width, overlay_height);

    // Clear background
    let clear = ratatui::widgets::Clear;
    frame.render_widget(clear, overlay_area);

    let mut lines = vec![
        Line::from(Span::styled("Select Chest Type", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
        Line::from(""),
    ];

    for (i, ct) in ChestType::ALL.iter().enumerate() {
        let unlocked = app.state.unlocked_chests.contains(ct);
        let is_selected = i == app.chest_menu_selected;
        let level_req = ct.required_level();

        let key_name = match ct {
            ChestType::Wooden => "None",
            ChestType::Iron => "Iron Key",
            ChestType::Silver => "Silver Key",
            ChestType::Gold => "Gold Key",
            ChestType::Crystal => "Crystal Key",
            ChestType::Shadow => "Shadow Key",
            ChestType::Void => "Void Key",
        };

        let has_level = app.state.player.level >= level_req;
        let has_key = match ct {
            ChestType::Wooden => true,
            ChestType::Iron => app.state.upgrades.get_level("iron_key") > 0,
            ChestType::Silver => app.state.upgrades.get_level("silver_key") > 0,
            ChestType::Gold => app.state.upgrades.get_level("gold_key") > 0,
            ChestType::Crystal => app.state.upgrades.get_level("crystal_key") > 0,
            ChestType::Shadow => app.state.upgrades.get_level("shadow_key") > 0,
            ChestType::Void => app.state.upgrades.get_level("void_key") > 0,
        };

        let marker = if is_selected { "\u{25b6} " } else { "  " };
        let number = format!("[{}] ", i + 1);

        let name_style = if is_selected {
            Style::default().fg(ct.color()).add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
        } else if unlocked {
            Style::default().fg(ct.color()).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let mut name_line = vec![
            Span::styled(marker, Style::default().fg(Color::Yellow)),
            Span::styled(number, Style::default().fg(Color::White)),
            Span::styled(ct.name(), name_style),
        ];

        if unlocked {
            name_line.push(Span::styled(" \u{2713}", Style::default().fg(Color::Green)));
        }

        lines.push(Line::from(name_line));

        // Show requirements if not unlocked
        if !unlocked {
            let mut req_parts = Vec::new();
            if !has_key && *ct != ChestType::Wooden {
                if !has_level {
                    req_parts.push(format!("{} ({})", key_name, level_req));
                } else {
                    req_parts.push(key_name.to_string());
                }
            } else if !has_level {
                req_parts.push(format!("Level {}", level_req));
            }
            let req_text = if req_parts.is_empty() {
                "Unlocked!".to_string()
            } else {
                format!("Need: {}", req_parts.join(", "))
            };

            lines.push(Line::from(vec![
                Span::raw("      "),
                Span::styled(req_text, Style::default().fg(Color::Red)),
            ]));
        }

        lines.push(Line::from(""));
    }

    lines.push(Line::from(Span::styled(
        "[↑↓] Navigate  [E] Select  [1-7] Quick Select  [C/Space/Esc] Close",
        Style::default().fg(Color::DarkGray),
    )));

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title(" Chest Selection ");
    let paragraph = Paragraph::new(lines).block(block);
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

fn draw_settings_overlay(frame: &mut Frame, app: &App, area: Rect) {
    if app.show_dev_options {
        draw_dev_options_overlay(frame, app, area);
    } else {
        draw_main_settings_overlay(frame, app, area);
    }
}

fn draw_main_settings_overlay(frame: &mut Frame, app: &App, area: Rect) {
    // Settings panel (centered and compact)
    let overlay_width = 50.min(area.width.saturating_sub(4));
    let overlay_height = 18.min(area.height.saturating_sub(4));
    let x = (area.width.saturating_sub(overlay_width)) / 2;
    let y = (area.height.saturating_sub(overlay_height)) / 2;
    let overlay_area = Rect::new(x, y, overlay_width, overlay_height);

    // Clear background for settings box
    let clear = ratatui::widgets::Clear;
    frame.render_widget(clear, overlay_area);

    let mut settings_lines = vec![
        Line::from(""),
        Line::from(""),
    ];

    // Setting 0: Volume slider
    let is_selected_0 = app.settings_selected == 0;
    let marker_0 = if is_selected_0 { "▶ " } else { "  " };
    let vol_pct = (app.setting_volume * 100.0).round() as u32;
    let filled = (app.setting_volume * 10.0).round() as usize;
    let empty = 10 - filled;
    let bar: String = "█".repeat(filled) + &"░".repeat(empty);

    let vol_spans = vec![
        Span::raw("   "),
        Span::styled(marker_0, Style::default().fg(Color::Yellow)),
        Span::styled("Volume: ", Style::default().fg(Color::White).add_modifier(
            if is_selected_0 { Modifier::BOLD } else { Modifier::empty() }
        )),
        Span::styled(
            format!("[{}] {}%", bar, vol_pct),
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ),
    ];
    settings_lines.push(Line::from(vol_spans));
    settings_lines.push(Line::from(""));

    // Setting 1: Animations toggle
    let anim_status = if app.setting_show_animations {
        "ON"
    } else {
        "OFF"
    };
    let anim_color = if app.setting_show_animations {
        Color::Green
    } else {
        Color::Red
    };
    let is_selected_1 = app.settings_selected == 1;
    let marker_1 = if is_selected_1 { "▶ " } else { "  " };

    settings_lines.push(Line::from(vec![
        Span::raw("   "),
        Span::styled(marker_1, Style::default().fg(Color::Yellow)),
        Span::styled("Animations: ", Style::default().fg(Color::White).add_modifier(
            if is_selected_1 { Modifier::BOLD } else { Modifier::empty() }
        )),
        Span::styled(
            anim_status,
            Style::default()
                .fg(anim_color)
                .add_modifier(Modifier::BOLD),
        ),
    ]));
    settings_lines.push(Line::from(""));

    // Setting 2: Chest Sounds toggle
    let chest_status = if app.setting_chest_sounds { "ON" } else { "OFF" };
    let chest_color = if app.setting_chest_sounds { Color::Green } else { Color::Red };
    let is_selected_2 = app.settings_selected == 2;
    let marker_2 = if is_selected_2 { "▶ " } else { "  " };

    settings_lines.push(Line::from(vec![
        Span::raw("   "),
        Span::styled(marker_2, Style::default().fg(Color::Yellow)),
        Span::styled("Chest Sounds: ", Style::default().fg(Color::White).add_modifier(
            if is_selected_2 { Modifier::BOLD } else { Modifier::empty() }
        )),
        Span::styled(
            chest_status,
            Style::default().fg(chest_color).add_modifier(Modifier::BOLD),
        ),
    ]));
    settings_lines.push(Line::from(""));

    // Setting 3: UI Sounds toggle
    let ui_status = if app.setting_ui_sounds { "ON" } else { "OFF" };
    let ui_color = if app.setting_ui_sounds { Color::Green } else { Color::Red };
    let is_selected_3 = app.settings_selected == 3;
    let marker_3 = if is_selected_3 { "▶ " } else { "  " };

    settings_lines.push(Line::from(vec![
        Span::raw("   "),
        Span::styled(marker_3, Style::default().fg(Color::Yellow)),
        Span::styled("UI Sounds: ", Style::default().fg(Color::White).add_modifier(
            if is_selected_3 { Modifier::BOLD } else { Modifier::empty() }
        )),
        Span::styled(
            ui_status,
            Style::default().fg(ui_color).add_modifier(Modifier::BOLD),
        ),
    ]));
    settings_lines.push(Line::from(""));

    // Setting 4: Dev Options
    let is_selected_4 = app.settings_selected == 4;
    let marker_4 = if is_selected_4 { "▶ " } else { "  " };

    settings_lines.push(Line::from(vec![
        Span::raw("   "),
        Span::styled(marker_4, Style::default().fg(Color::Yellow)),
        Span::styled("Dev Options", Style::default()
            .fg(Color::Magenta)
            .add_modifier(if is_selected_4 {
                Modifier::BOLD | Modifier::UNDERLINED
            } else {
                Modifier::empty()
            })),
    ]));
    settings_lines.push(Line::from(""));
    settings_lines.push(Line::from(""));

    settings_lines.push(Line::from(Span::styled(
        " [↑↓] Navigate  [E] Select  [Esc] Close",
        Style::default().fg(Color::DarkGray),
    )));

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title(" Settings ");
    let paragraph = Paragraph::new(settings_lines).block(block);
    frame.render_widget(paragraph, overlay_area);
}

fn draw_dev_options_overlay(frame: &mut Frame, app: &App, area: Rect) {
    // Dev options panel (centered and larger)
    let overlay_width = 50.min(area.width.saturating_sub(4));
    let overlay_height = 18.min(area.height.saturating_sub(4));
    let x = (area.width.saturating_sub(overlay_width)) / 2;
    let y = (area.height.saturating_sub(overlay_height)) / 2;
    let overlay_area = Rect::new(x, y, overlay_width, overlay_height);

    // Clear background
    let clear = ratatui::widgets::Clear;
    frame.render_widget(clear, overlay_area);

    let mut lines = vec![
        Line::from(""),
        Line::from(""),
    ];

    let options = [
        ("Reset Game", "Delete all progress", Color::Red),
        ("Unlock All Chests", "Unlock all chest types", Color::Yellow),
        ("Max Money", "Set GP to 999,999,999", Color::Yellow),
        ("Max Skills", "Set skill points to 9999", Color::Cyan),
        ("Max Essence", "Set essence to 999,999", Color::Magenta),
    ];

    for (i, (name, desc, color)) in options.iter().enumerate() {
        let is_selected = app.dev_option_selected == i;
        let marker = if is_selected { "▶ " } else { "  " };

        lines.push(Line::from(vec![
            Span::raw("      "),
            Span::styled(marker, Style::default().fg(Color::Yellow)),
            Span::styled(*name, Style::default()
                .fg(*color)
                .add_modifier(if is_selected {
                    Modifier::BOLD | Modifier::UNDERLINED
                } else {
                    Modifier::empty()
                })),
        ]));
        lines.push(Line::from(Span::styled(
            format!("        {}", desc),
            Style::default().fg(Color::DarkGray),
        )));
    }

    lines.push(Line::from(""));

    // Controls at bottom (inside box, centered)
    lines.push(Line::from(Span::styled(
        "  [↑↓] Navigate  [E] Select  [Esc] Close",
        Style::default().fg(Color::DarkGray),
    )));

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Magenta))
        .title(" Dev Options ");
    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, overlay_area);
}
