extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

// see https://stackoverflow.com/questions/48071513/how-to-use-one-module-from-another-module-in-a-rust-cargo-project/48071730#48071730
// on combining modules and using sibling modules
mod ui;
mod company;
mod config;
mod file_io;
mod account;
mod transaction;
mod crossterm;
mod app;

use std::{
    error::Error,
    io,
};
use ::crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    Terminal,
    backend::CrosstermBackend,
};

use clap::Parser;

use crate::company::Company;
use crate::account::Account;
use crate::transaction::Transaction;
use crate::app::App;
use crate::crossterm::run_app;
use crate::config::{Config, Args};


fn main() -> Result<(), Box<dyn Error>> {

    let args = Args::parse();
    let config = Config::default()
        .database(args.database)
        .tick_rate(args.tick_rate);

    // setup the terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create and start app
    let app = App::default()
        .database(config.database.clone());
    let res = run_app(&mut terminal, app, config);

    // restore the terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())

    //let mut copy_accounts = company.accounts.clone();
    //println!();

    // company.sort_accounts("asc");

    // for account in company.accounts.iter() {
    //     println!("{} {} {} {} {:?} {:?}",
    //         account.id,  
    //         account.name,
    //         account.r#type,
    //         account.parent,
    //         account.transactions,
    //         account.subaccounts 
    //     )
    // }

    // for transaction in company.transactions.iter() {
    //     println!("{} {} {} {:?} {} {}",
    //         transaction.id,
    //         transaction.debit, 
    //         transaction.credit,
    //         transaction.date,      
    //         transaction.amount,
    //         transaction.memo

    //     )
    // }

    // let mut new_account = Account::new();
    // new_account.set_id_in_company(&mut company)
    //            .set_parent("2")
    //            .set_type_in_company("d", &company)
    //            .set_name("Something");

    // let mut new_transaction = Transaction::new();
    // new_transaction.set_id_in_company(&mut company)
    //                .set_credit("1")
    //                .set_debit("2")
    //                .set_amount("1002.33")
    //                .set_memo("Hello");

    // company.write_to(config.get_database()).unwrap_or_else(|err| {
    //     eprintln!("Problem saving company database: {}", err);
    //     process::exit(1);
    // });

}
