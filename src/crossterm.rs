use crossterm::{
    event::{self, Event, KeyCode},
    execute,
};

use tui::{
    Terminal, Frame,
    backend::{Backend, CrosstermBackend},
};

use std::{
    error::Error,
    io,
    time::{Duration, Instant}
};

use crate::app::{App, InputMode};
use crate::ui::ui;


pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
     mut app: App, 
     tick_rate: Duration
) -> io::Result<()> {

    let mut last_tick = Instant::now();
    loop {

        // drawing wants a closure to draw a frame each call.
        // include the state to draw with the closure
        terminal.draw(|f| ui(f, &app))?;

        
        let timeout = tick_rate
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
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}