
use chrono::prelude::*;
use std::error::Error;

use crate::file_io;


// This the primary object that contains the Accounts and Tranactions
#[derive(Debug, Serialize, Deserialize)]
pub struct Company {
    pub accounts: Vec<Account>,
    pub transactions: Vec<Transaction>
}

impl Company {
    pub fn from(db_path: &str) -> Result<Self, Box<dyn Error>> {
        let db_data = file_io::read(db_path)?;
        let res: Company = serde_json::from_str(&db_data[..])?;

        Ok(res)
    }
}

// Accounts are entities that have Transactions
#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub subaccounts: Vec<String>,
    pub name: String,
    pub r#type: String,
    pub transactions: Vec<String>,
    pub parent: String

}

impl Account {

}

// Transactions move money between accounts
#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub credit: String,
    pub debit: String,
    pub amount: String,
    pub memo: String,
    pub date: NaiveDate
    // see https://docs.rs/chrono/0.4.19/chrono/
    // for date related things
}

impl Transaction {

}
