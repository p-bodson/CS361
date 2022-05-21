use std::{
    error::Error,
    io,
    time::{Duration, Instant}
};

use tui::{
    Frame,
    backend::Backend,
    style::{Color, Modifier, Style},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, ListItem, Paragraph, List, Gauge, Wrap},
};

use unicode_width::UnicodeWidthStr;

use crate::company::Company;
use crate::account::Account;
use crate::transaction::Transaction;
use crate::app::{App, InputMode, Focus};


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

    draw_text(f, app, chunks[0]);
    draw_input(f, app, chunks[1]);
    draw_messages(f, app, chunks[2]);
    draw_gauges(f, app, chunks[3]);
}

fn draw_text<B>(f: &mut Frame<B>, app: &App, area: Rect) 
where
    B: Backend,
{

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(100), 
            ].as_ref(),
        )
        .split(area);

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
}

fn draw_gauges<B>(f: &mut Frame<B>, app: &App, area: Rect) 
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(100), 
            ].as_ref(),
        )
        .split(area);

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
    f.render_widget(gauge, chunks[0]);
}

fn draw_input<B>(f: &mut Frame<B>, app: &App, area: Rect) 
where
    B: Backend,
{

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(100), 
            ].as_ref(),
        )
        .split(area);


    let input = Paragraph::new(app.input.as_ref())
    .style(match app.input_mode {
        InputMode::Normal => Style::default(),
        InputMode::Editing => Style::default().fg(Color::Yellow),
    })
    .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunks[0]);

    match app.input_mode {
        InputMode::Normal =>
            // hide cursor. frame does by default
            {}

        InputMode::Editing => {
            // show cursor and place at specific coordinates
            f.set_cursor(
                chunks[0].x + app.input.width() as u16 +1,
                chunks[0].y + 1,
            )
        }
    }
}

fn draw_messages<B>(f: &mut Frame<B>, app: &App, area: Rect) 
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints(
            [
                Constraint::Length(55),
                Constraint::Min(10),
                Constraint::Length(55),
            ].as_ref(),
        )
        .split(area);

    let mut text = Text::from("(b) List the current balance for your portfolio\n");
    text.extend(Text::raw("(t) Enter a new transaction\n"));
    text.extend(Text::raw("(r) Examine the register for an account\n"));
    text.extend(Text::raw("(p) See the Profit and Loss for a time period\n"));
    text.extend(Text::raw("(l) List the chart of accounts\n"));
    text.extend(Text::raw("(g) Generate an expense report\n"));
    text.extend(Text::raw("(q) Quit the program\n"));

    let menu = Paragraph::new(text);
    f.render_widget(menu, chunks[0]);


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


    // print out the accounts in a window
    let text = fill_viewer(app);
    let viewer = Paragraph::new(text)
        .block(Block::default().title("Viewer").borders(Borders::ALL))
        .wrap(Wrap { trim: false});
    f.render_widget(viewer, chunks[1]);
}

fn fill_viewer(app: &App) -> Text {

    let mut text = Text::from("");

    match app.focus {
        Focus::Charts => {
            text.extend(Text::raw("Chart of Accounts"));
            let chart = app.company.get_chart_of_accounts().unwrap();
            for listing in chart {
                let mut line = String::new();
                let mut first = true;
                for account in listing {
                    if first {
                        line = format!("{}", account.name);
                        first = false;
                    }
                    else {
                        line = format!("{} -> {}", account.name, line );
                    }
        
                }
                text.extend(Text::raw(line));
            }
        },
        Focus::ExpenseReport => {
            text.extend(Text::raw("Generating Expense Report"));
        },
        Focus::BalanceSheet => {
            text.extend(Text::raw("List the current balance for your portfolio"));
        },
        Focus::NewTransaction => {
            text.extend(Text::raw("Enter a new transaction"));
        },
        Focus::Register => {
            text.extend(Text::raw("Examine the register for an account"));
        },
        Focus::ProfitAndLoss => {
            text.extend(Text::raw("See the Profit and Loss for a time period"));
        },
        Focus::Nothing => {},
    }

    text
}