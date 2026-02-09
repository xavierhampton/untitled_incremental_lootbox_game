mod animation;
mod app;
mod data;
mod event;
mod game;
mod ui;

use std::io;

use app::App;
use color_eyre::Result;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use event::EventHandler;
use ratatui::prelude::*;

fn main() -> Result<()> {
    color_eyre::install()?;

    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    let mut app = App::new();
    let mut event_handler = EventHandler::new(33); // ~30 ticks/sec

    loop {
        terminal.draw(|frame| ui::draw(frame, &app))?;

        match event_handler.next()? {
            event::Event::Tick => app.on_tick(),
            event::Event::Key(key) => {
                if app.on_key(key) {
                    break;
                }
            }
        }
    }

    app.save_game();
    Ok(())
}
