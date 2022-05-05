use std::process;
use std::io;
use std::io::Write;
use std::env;
use config::Config;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use chrono::prelude::*;


fn main() {

    // get the arguments from the command line
    let args: Vec<String> = env::args().collect();
    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Problem getting user input: {}", err);
        process::exit(1);
    });

    // attempt to open the database
    let db_data = file_io::read(config.get_database()).unwrap_or_else(|err| {
        eprintln!("Problem reading database file: {}", err);
        process::exit(1);
    });

    let p: Company = serde_json::from_str(&db_data[..]).unwrap_or_else(|err| {
        eprintln!("Problem parsing database JSON file: {}", err);
        process::exit(1);
    });

    for account in p.accounts.iter() {
        println!("{} {:?} {} {} {:?} {}",
            account.id, 
            account.subaccounts,        
            account.name,
            account.r#type,
            account.transactions,
            account.parent
        )
    }

    for tranaction in p.transactions.iter() {
        println!("{} {} {} {} {} {:?}",
            tranaction.id, 
            tranaction.credit,        
            tranaction.debit,
            tranaction.amount,
            tranaction.memo,
            tranaction.date
        )
    }

    

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


#[derive(Debug, Serialize, Deserialize)]
struct Company {
    accounts: Vec<Account>,
    transactions: Vec<Transaction>
}


#[derive(Debug, Serialize, Deserialize)]
struct Account {
    id: String,
    subaccounts: Vec<String>,
    name: String,
    r#type: String,
    transactions: Vec<String>,
    parent: String

}

impl Account {

}

// see https://docs.rs/chrono/0.4.19/chrono/
// for DateTime related things

#[derive(Debug, Serialize, Deserialize)]
struct Transaction {
    id: String,
    credit: String,
    debit: String,
    amount: String,
    memo: String,
    date: NaiveDate
}

impl Transaction {

}


mod config {
    // see https://doc.rust-lang.org/std/collections/struct.HashMap.html
    // on using HashMaps
    use std::collections::HashMap;
    use std::error::Error;

    #[derive(Debug)]
    pub struct Config {
        database: String,
    }

    impl Config {
        pub fn new(args: Vec<String>) -> Result<Self, Box<dyn Error>> {
            let arg_values = Config::parse_args(&args)?;
            let database = &arg_values["database"];

            Ok(Config {
                database: database.to_string(),
            })
        }

        pub fn get_database(&self) -> &str {
            &self.database[..]
        }

        fn parse_args(args: &Vec<String>) -> Result<HashMap<String, &String>, Box<dyn Error>> {           
            let mut arg_values = HashMap::new();
            arg_values.insert("database".to_string(), &args[1]);
            Ok(arg_values)
        }
    }

}

mod file_io {
    use std::error::Error;
    use std::fs;
    use std::fs::OpenOptions;
    use std::io::Write;

    pub fn read(file_path: &str) -> Result<String, Box<dyn Error>> {
        let utf8_vector = fs::read(file_path)?;
        let foo = String::from_utf8_lossy(&utf8_vector).to_string();
    
        Ok(foo)
    }
    
    pub fn write(file_path: &str, thing_to_write: &str) -> Result<(), Box<dyn Error>> {
        let mut f = OpenOptions::new().write(true).append(true).open(file_path)?;
    
        f.write(thing_to_write.as_bytes())?;
        f.write("\n".as_bytes())?;
        
        Ok(())
    }

    //pub fn truncate(file_path: &str) -> Result<(), Box<dyn Error>> {
    //    OpenOptions::new().write(true).create(true).truncate(true).open(file_path)?;
    //    Ok(())
    //}

}

mod ui {
    use std::io;

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
}