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
