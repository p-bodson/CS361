use std::{
    error::Error,
    io,
    time::{Duration, Instant},
    path::PathBuf,
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

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {

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

fn draw_messages<B>(f: &mut Frame<B>, app: &mut App, area: Rect) 
where
    B: Backend,
{
    // make the layout
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

    // print out the help text
    let text = get_menu_text(app);
    let menu = Paragraph::new(text);
    f.render_widget(menu, chunks[0]);


    // print out the message window
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


    // print out the the main viewer
    let text = fill_viewer(app);
    let viewer = Paragraph::new(text)
        .block(Block::default().title("Viewer").borders(Borders::ALL))
        .wrap(Wrap { trim: false});
    f.render_widget(viewer, chunks[1]);

}

fn get_menu_text(app: &mut App) -> Text {
    let mut text = Text::from("(b) List the current balance for your portfolio\n");
    text.extend(Text::raw("(t) Enter a new transaction\n"));
    text.extend(Text::raw("(r) Examine the register for an account\n"));
    text.extend(Text::raw("(d) Delete a transaction\n"));
    text.extend(Text::raw("(l) List the chart of accounts\n"));
    text.extend(Text::raw("(g) Generate an expense report\n"));
    text.extend(Text::raw("(q) Quit the program\n"));
    
    text
}

fn fill_viewer(app: &mut App) -> Text {

    let mut text = Text::from("");

    match app.focus {
        Focus::Charts => {
            text = show_charts(app);
        },
        Focus::ExpenseReport => {
            text = show_expense_report(app);
        },
        Focus::BalanceSheet => {
            text = show_balance_sheet(app);
        },
        Focus::NewTransaction => {
            text.extend(Text::raw("Enter a new transaction"));
        },
        Focus::Register => {
            text.extend(Text::raw("Examine the register for an account"));
        },
        Focus::DeleteTransaction => {
            text.extend(Text::raw("Deleting Transaction"));
        },
        Focus::Nothing => {
            app.report_gen_done = false;
        },
    }

    text
}

fn show_balance_sheet(app: &mut App) -> Text {
    let mut text = Text::from("");

    let balances = app.company.get_balance_summary();
    if balances.is_none() {
        text.extend(Text::raw("No balances to show"));
        return text; 
    }

    text.extend(Text::raw("Balance Sheet"));
    let balances = balances.unwrap();

    // first print out the debits
    // then the credits
    let mut printed_debits = false;
    let mut printed_credits = false;
    let mut total_debits = 0.0;
    let mut total_credits = 0.0;

    for x in 0..2 {
        for balance in &balances {
            let (account, sum) = balance;
            if account.r#type == "d" && x == 0 {
                if !printed_debits {
                    text.extend(Text::raw("Debits"));
                    printed_debits = true;
                }
                let line = String::from(format!("{}\n    = {:.2}", account.name, sum));
                text.extend(Text::raw(line));
                total_debits += sum; 
            }
            else if account.r#type == "c" && x == 1 {
                if !printed_credits {
                    text.extend(Text::raw("Credits"));
                    printed_credits = true;
                }
                let line = String::from(format!("{}\n    = {:.2}", account.name, sum));
                text.extend(Text::raw(line));
                total_credits += sum; 
            }
        }
    }

    text.extend(Text::raw(format!("Total Debits = {}", total_debits)));
    text.extend(Text::raw(format!("Total Cedits = {}", total_credits)));

    text
}

fn show_charts(app: &mut App) -> Text {

    let mut text = Text::from("");

    let chart = app.company.get_chart_of_accounts();
    if chart.is_none() {
        text.extend(Text::raw("No accounts to show"));
        return text;
    }

    text.extend(Text::raw("Chart of Accounts"));
    let chart = chart.unwrap();
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
    };

    text
}

fn show_expense_report(app: &mut App) -> Text {
   
    let mut text = Text::from("");

    if !app.report_gen_done {
        text.extend(Text::raw("Generating Expense Report at"));
        let res = app.company.generate_expense_report(&app.db_path);
        if res.is_err() {
            text.extend(Text::raw("error generating report")); 
        }
        else {
            let res = res.unwrap();
            if res.is_none() {
                text.extend(Text::raw("No Path To Report")); 
            }
            else {
                let path = res.unwrap();
                app.report_path = path.clone();
                text.extend(Text::raw(format!("{}", path.display())));
                text.extend(Text::raw("Locate the expense report at the above path"));
            }

        }
        app.report_gen_done = true;
    }
    else {
        text.extend(Text::raw("Path to report is ..."));
        let path = app.report_path.clone();
        text.extend(Text::raw(format!("{}", path.display())));
    }

    text
}