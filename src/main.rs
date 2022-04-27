use std::process;
use std::io;
use std::io::Write;

fn main() {

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

        let command = ui::capture_input().unwrap_or_else(|err| {
            eprintln!("Problem getting user input: {}", err);
            process::exit(1);
        })
        .into_owned();


        if command == "q\n" {
            println!("");
            print!("{}", ui::farewell());
            break;
        }

        match &command[..] {
            "b\n" => println!("{}", command),
            "t\n" => println!("{}", command),
            "r\n" => println!("{}", command),
            "p\n" => println!("{}", command),
            _   => { 
                println!{"I don't know that command, please see the Features for the known commands."};
                continue;
            },
        }
        
    }
}

mod ui {
    use std::io;
    use std::borrow::Cow;

    pub fn capture_input<'a>() -> io::Result<Cow<'a, str>> {
        // see https://doc.rust-lang.org/std/io/struct.Stdin.html
        // for origin of this code on reading user input
        
        let mut buffer =  String::new();
        io::stdin().read_line(&mut buffer)?;


        // see https://hermanradtke.com/2015/05/29/creating-a-rust-function-that-returns-string-or-str.html/
        // for information on why a Cow is returned.
        Ok(Cow::Owned(buffer))
    }
    

    pub fn welcome() -> String {
        format!("{}\n\n{}\n",
            "Welcome to Money, the double-entry ledger app for counting your wealth.",
            "Type the letter in the parentheses to perform the corresponding feature."
        )      
    }

    pub fn features() -> String {
        format!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
            "====================================================",
            "    Features:",
            "    (b) List the current balance for your portfolio",
            "    (t) Enter a new transaction",
            "    (r) Examine the register for an account",
            "    (p) See the Profit and Loss for a time period",
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