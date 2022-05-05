use std::process;
use std::io;
use std::io::Write;
use std::env;

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

fn main() {

    // get the arguments from the command line
    let args: Vec<String> = env::args().collect();
    let config = config::Config::new(args).unwrap_or_else(|err| {
        eprintln!("Problem getting user input: {}", err);
        process::exit(1);
    });

    // attempt to load the database
    let mut company = company::Company::from(config.get_database()).unwrap_or_else(|err| {
        eprintln!("Problem loading company database: {}", err);
        process::exit(1);
    });

    //let mut copy_accounts = company.accounts.clone();
    //println!();

    company.sort_accounts("asc");

    for account in company.accounts.iter() {
        println!("{} {:?} {} {} {:?} {}",
            account.id, 
            account.subaccounts,        
            account.name,
            account.r#type,
            account.transactions,
            account.parent
        )
    }

    for tranaction in company.transactions.iter() {
        println!("{} {} {} {} {} {:?}",
            tranaction.id, 
            tranaction.credit,        
            tranaction.debit,
            tranaction.amount,
            tranaction.memo,
            tranaction.date
        )
    }

    company.write_to(config.get_database()).unwrap_or_else(|err| {
        eprintln!("Problem saving company database: {}", err);
        process::exit(1);
    });

    

    println!("");
    println!("{}", ui::welcome());

    // the interactive mode loop
    loop {
        println!("{}", ui::features());

        // capture user input

        print!("> ");
        // see https://doc.rust-lang.org/std/macro.print.html
        // on flushing stdout
        io::stdout().flush().unwrap();

        let command: String = ui::capture_input().unwrap_or_else(|err| {
            eprintln!("Problem getting user input: {}", err);
            process::exit(1);
        }).split_whitespace().collect();


        if command == "q" {
            println!("");
            print!("{}", ui::farewell());
            break;
        }

        match &command[..] {
            "b" => println!("{}: printing balance sheet", command),
            "t" => println!("{}: entering a transaction", command),
            "r" => println!("{}: querying a register", command),
            "p" => println!("{}: printing an income statement", command),
            "l" => println!("{}: listing the chart of accounts", command),
            _   => { 
                println!{"I don't know that command, please see the Features for the known commands."};
                continue;
            },
        }
        
    }
}
