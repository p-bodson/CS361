
use std::{
    error::Error,
    io,
    time::{Duration, Instant}
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use tui::{
    Terminal, Frame,
    backend::{Backend, CrosstermBackend},
    style::{Color, Modifier, Style},
    layout::{Alignment, Constraint, Direction, Layout},
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, ListItem, Paragraph, List, Gauge},
};

use unicode_width::UnicodeWidthStr;

use crate::company::Company;
use crate::account::Account;
use crate::transaction::Transaction;
use crate::app::{App, InputMode};


pub fn capture_input<'a>() -> io::Result<String> {
    // see https://doc.rust-lang.org/std/io/struct.Stdin.html
    // for origin of this code on reading user input
    
    let mut buffer =  String::new();
    io::stdin().read_line(&mut buffer)?;

    let output = buffer.clone();

    Ok(output)
}


pub fn welcome() -> String {
    format!("{}\n\n{}\n",
        "Welcome to Money, the double-entry ledger app for counting your wealth.",
        "Type the letter in the parentheses to perform the corresponding feature."
    )      
}

pub fn features() -> String {
    format!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
        "====================================================",
        "    Features:",
        "    (b) List the current balance for your portfolio",
        "    (t) Enter a new transaction",
        "    (r) Examine the register for an account",
        "    (p) See the Profit and Loss for a time period",
        "    (l) List the chart of accounts",
        "    (q) Quit the program",
        "===================================================="
    )
}

pub fn farewell() -> String {
    format!("{}\n",
        "Thank you, goodbye."
    )
}

pub fn show_register(account_id: &str, company: &Company) {

    let transactions = company.get_transactions_by_account(account_id);

    if transactions.is_none() {
        println!("Account_id {} cannot be found", account_id);
    } 
    else {
        println!("Showing register for account_id {}", account_id);

        let transactions = transactions.unwrap();

        for transaction in transactions.iter() {
            println!("{:?}", transaction);
        }
    }
}


pub fn show_chart_of_accounts(company: &Company) {

    let accounts = company.get_accounts();

    if accounts.is_none() {
        println!("No Accounts To Show");
    } 
    else {
        println!("Showing Chart of Accounts");

        let accounts = accounts.unwrap();

        for account in accounts.iter() {
            println!("{:?}", account);
        }
    }
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1), 
                Constraint::Length(3), 
                Constraint::Min(1),
                Constraint::Length(3),
            ].as_ref(),
        )
        .split(f.size());

    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing."),
            ],
            Style::default().add_modifier(Modifier::SLOW_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record the message"),
            ],
            Style::default(),
        )
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunks[1]);
    match app.input_mode {
        InputMode::Normal =>
            // hide cursor. frame does by default
            {}

        InputMode::Editing => {
            // show cursor and place at specific coordinates
            f.set_cursor(
                chunks[1].x + app.input.width() as u16 +1,
                chunks[1].y + 1,
            )
        }
    }


    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
            ListItem::new(content)
        })
        .collect();

    let messages = 
        List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages").border_type(BorderType::Thick));
    f.render_widget(messages, chunks[2]);


    let gauge_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(50), 
                Constraint::Percentage(50), 
            ].as_ref(),
        )
        .split(chunks[3]);

    let label = Span::styled(
        format!("{:.2}%", app.progress * 100.0),
        Style::default()
            .fg(Color::Red)
            .add_modifier(Modifier::ITALIC | Modifier::BOLD),
    );
    let gauge = Gauge::default()
        .block(Block::default().title("Gauge1").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Yellow))
        .ratio(app.progress)
        .label(label)
        .use_unicode(true);
    f.render_widget(gauge, gauge_chunks[0]);

    let label = Span::styled(
        format!("{:.2}%", app.progress * 100.0),
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::ITALIC | Modifier::BOLD),
    );
    let gauge = Gauge::default()
        .block(Block::default().title("Gauge2").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Red))
        .ratio(app.progress)
        .label(label)
        .use_unicode(true);
    f.render_widget(gauge, gauge_chunks[1]);
}
