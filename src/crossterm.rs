// see https://github.com/fdehau/tui-rs/tree/master/examples
// and https://docs.rs/tui/0.18.0/tui/index.html
// for info an creating a Text User Interface

use crossterm::{
    event::{self, Event, KeyCode},
};

use tui::{
    Terminal,
    backend::Backend,
};

use std::{
    io,
    time::{Duration, Instant}
};

use crate::app::{App, InputMode, Focus};
use crate::ui::ui;
use crate::config::Config;


pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
     mut app: App, 
     config: Config,
) -> io::Result<()> {

    let mut last_tick = Instant::now();

    // attempt to load the database
    app = app.load_company()?;

    loop {

        // drawing wants a closure to draw a frame each call.
        // include the state to draw with the closure
        terminal.draw(|f| ui(f, &mut app))?;

        
        let timeout = config.tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0)); // times out on error

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('e') => {
                            app.input_mode = InputMode::Editing;
                        }
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        KeyCode::Char('l') => {
                            app.toggle_focus(Focus::Charts);
                        }
                        KeyCode::Char('g') => {
                            app.toggle_focus(Focus::ExpenseReport);
                        }
                        KeyCode::Char('b') => {
                            app.toggle_focus(Focus::BalanceSheet);
                        }
                        KeyCode::Char('t') => {
                            app.toggle_focus(Focus::NewTransaction);
                        }
                        KeyCode::Char('r') => {
                            app.toggle_focus(Focus::Register);
                        }
                        KeyCode::Char('d') => {
                            app.toggle_focus(Focus::DeleteTransaction);
                        }
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Enter => {
                            app.messages.push(app.input.drain(..).collect());
                        }
                        KeyCode::Char(c) => {
                            app.input.push(c);
                        }
                        KeyCode::Backspace => {
                            app.input.pop();
                        }
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                        }
                        _ => {}
                    },
                }
            }
        }
        if last_tick.elapsed() >= config.tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}