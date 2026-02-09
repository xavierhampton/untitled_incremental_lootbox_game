use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Tabs};
use ratatui::Frame;

use crate::app::{ActiveTab, App};
use super::tabs;

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(60, 60, 80)))
        .title(Span::styled(
            " Menu ",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // tab bar
            Constraint::Length(1), // separator
            Constraint::Min(1),   // content
        ])
        .split(inner);

    // Tab bar
    let titles: Vec<Line> = ActiveTab::ALL
        .iter()
        .map(|t| {
            let style = if *t == app.active_tab {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            Line::from(Span::styled(t.label(), style))
        })
        .collect();

    let selected = ActiveTab::ALL
        .iter()
        .position(|&t| t == app.active_tab)
        .unwrap_or(0);

    let tabs = Tabs::new(titles)
        .select(selected)
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .divider("\u{2502}");

    frame.render_widget(tabs, sections[0]);

    // Separator line under tabs
    let sep_width = sections[1].width as usize;
    let sep = "\u{2500}".repeat(sep_width);
    frame.render_widget(
        Paragraph::new(Span::styled(sep, Style::default().fg(Color::Rgb(60, 60, 80)))),
        sections[1],
    );

    // Tab content
    match app.active_tab {
        ActiveTab::Skills => tabs::skills::draw(frame, app, sections[2]),
        ActiveTab::Upgrades => tabs::upgrades::draw(frame, app, sections[2]),
        ActiveTab::Relics => tabs::relics::draw(frame, app, sections[2]),
        ActiveTab::Inventory => tabs::inventory::draw(frame, app, sections[2]),
        ActiveTab::Stats => tabs::stats::draw(frame, app, sections[2]),
        ActiveTab::Rebirth => tabs::rebirth::draw(frame, app, sections[2]),
    }
}
