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
use nustat_core::net::stat::{NetStatStrage, NetStatData};
use crate::{app::App, sys, ui};

pub fn run(tick_rate: Duration, enhanced_graphics: bool, netstat_strage: &mut Arc<NetStatStrage>) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app_title: String = sys::get_app_title();
    let app = App::new(&app_title, enhanced_graphics);
    let res = run_app(&mut terminal, app, tick_rate, netstat_strage);

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
    tick_rate: Duration,
    netstat_strage: &mut Arc<NetStatStrage>
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let mut data_last_tick = Instant::now();
    loop {

        if data_last_tick.elapsed() >= Duration::from_millis(1000) {
            let diff_clone: NetStatData = netstat_strage.clone_data_and_reset();
            app.netstat_data.merge(diff_clone);
            app.remote_hosts = app.netstat_data.get_remote_hosts(None);
            //app.top_processes = app.netstat_data.get_top_processes();
            app.connections = app.netstat_data.get_connections(None);
            data_last_tick = Instant::now();
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
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
        if app.should_quit {
            return Ok(());
        }
    }
}