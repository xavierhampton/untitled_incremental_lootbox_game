mod animation;
mod app;
mod audio;
mod data;
#[cfg(not(target_arch = "wasm32"))]
mod event;
mod game;
mod input;
mod ui;

// ── Native entry point ──────────────────────────────────────────────────────

#[cfg(not(target_arch = "wasm32"))]
fn main() -> color_eyre::Result<()> {
    use std::io;

    use app::App;
    use color_eyre::Result;
    use crossterm::{
        execute,
        terminal::{
            EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
        },
    };
    use event::EventHandler;
    use input::GameKeyEvent;
    use ratatui::prelude::*;

    color_eyre::install()?;

    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = (|| -> Result<()> {
        let mut app = App::new();
        let mut event_handler = EventHandler::new(33); // ~30 ticks/sec

        loop {
            // Track terminal size
            if let Ok(size) = terminal.size() {
                app.set_screen_size(size.width, size.height);
            }

            terminal.draw(|frame| ui::draw(frame, &app))?;

            match event_handler.next()? {
                event::Event::Tick => app.on_tick(),
                event::Event::Key(key) => {
                    let game_key: GameKeyEvent = key.into();
                    if app.on_key(game_key) {
                        break;
                    }
                }
            }
        }

        app.save_game();
        Ok(())
    })();

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

// ── WASM entry point ────────────────────────────────────────────────────────

#[cfg(target_arch = "wasm32")]
fn main() -> std::io::Result<()> {
    use std::cell::RefCell;
    use std::panic::{AssertUnwindSafe, catch_unwind};
    use std::rc::Rc;

    use app::App;
    use input::GameKeyEvent;
    use ratzilla::{DomBackend, WebRenderer};

    console_error_panic_hook::set_once();

    let app = Rc::new(RefCell::new(App::new()));

    let backend = DomBackend::new()?;
    let terminal = ratzilla::ratatui::Terminal::new(backend)?;

    // Key events
    terminal.on_key_event({
        let app = app.clone();
        move |key_event| {
            let game_key: GameKeyEvent = key_event.into();
            let mut app = app.borrow_mut();
            if app.on_key(game_key) {
                app.save_game();
            }
        }
    });

    // Render loop — fires once per requestAnimationFrame (~60fps)
    // We drive game ticks at ~30/sec using a time accumulator.
    let last_time = Rc::new(RefCell::new(None::<f64>));
    let tick_accum = Rc::new(RefCell::new(0.0f64));
    const TICK_MS: f64 = 1000.0 / 30.0; // ~33.3ms per tick
    const MAX_TICKS_PER_FRAME: u32 = 5; // cap catch-up when tab is backgrounded

    terminal.draw_web({
        let app = app.clone();
        move |frame| {
            // Wrap in catch_unwind so panics during resize don't kill the
            // requestAnimationFrame loop — the next frame will recover.
            let _ = catch_unwind(AssertUnwindSafe(|| {
                let area = frame.area();

                // Skip rendering if area is too small
                if area.width < 20 || area.height < 10 {
                    return;
                }

                // Drive ticks via Performance.now()
                let now = web_sys::window()
                    .and_then(|w| w.performance())
                    .map(|p| p.now())
                    .unwrap_or(0.0);

                let mut last = last_time.borrow_mut();
                let mut accum = tick_accum.borrow_mut();

                if let Some(prev) = *last {
                    let dt = (now - prev).min(MAX_TICKS_PER_FRAME as f64 * TICK_MS);
                    *accum += dt;
                }
                *last = Some(now);

                // Set screen size and run ticks
                {
                    let mut app = app.borrow_mut();
                    app.set_screen_size(area.width, area.height);

                    let mut ticks = 0u32;
                    while *accum >= TICK_MS && ticks < MAX_TICKS_PER_FRAME {
                        app.on_tick();
                        *accum -= TICK_MS;
                        ticks += 1;
                    }
                }

                let app = app.borrow();
                ui::draw(frame, &app);
            }));
        }
    });

    Ok(())
}
