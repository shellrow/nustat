use std::{
    error::Error, io, time::{Duration, Instant}
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::sync::Arc;
use nustat_core::{config::AppConfig, net::stat::NetStatStrage};
use crate::{app::App, sys, ui};

pub fn run(app_config: AppConfig, enhanced_graphics: bool, netstat_strage: &mut Arc<NetStatStrage>) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app_title: String = sys::get_app_title();
    let app = App::new(&app_title, enhanced_graphics, app_config);
    let res = run_app(&mut terminal, app, netstat_strage);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    netstat_strage: &mut Arc<NetStatStrage>
) -> io::Result<()> {
    let tick_rate = Duration::from_millis(app.config.display.tick_rate);
    let mut last_tick = Instant::now();
    loop {

        if last_tick.elapsed() >= tick_rate {
            app.on_tick(netstat_strage.clone_data_and_reset());
            last_tick = Instant::now();
        }

        terminal.draw(|f| ui::draw(f, &mut app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Left | KeyCode::Char('a') => app.on_left(),
                        KeyCode::Up | KeyCode::Char('w') => app.on_up(),
                        KeyCode::Right | KeyCode::Char('d') => app.on_right(),
                        KeyCode::Down | KeyCode::Char('s') => app.on_down(),
                        KeyCode::Char(c) => app.on_key(c),
                        _ => {}
                    }
                }
            }
        }
        
        if app.should_quit {
            return Ok(());
        }
    }
}