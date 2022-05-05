
use chrono::prelude::*;
use std::error::Error;

use std::cmp::Ordering;

use crate::file_io;

// see https://www.youtube.com/watch?v=hIi_UlyIPMg
// on using serde to derive structs from json


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

    pub fn write_to(&self, db_path: &str) -> Result<(), Box<dyn Error>> {
        file_io::truncate(db_path)?;

        let content = serde_json::to_string(self)?;

        file_io::write(db_path, &content[..])?;

        Ok(())
    }

    pub fn insert_account(&mut self, account: Account) {       
        self.accounts.push(account);
    }

    pub fn delete_account(&mut self, to_delete: &Account) {

        let mut idx_to_delete: isize = -1;

        for (idx, account) in self.accounts.iter().enumerate() {
            if account.cmp(to_delete) == Ordering::Equal {
                idx_to_delete = idx.try_into().unwrap();
            }
        }

        if idx_to_delete >= 0 {
            self.accounts.remove(idx_to_delete.try_into().unwrap());
        }
    }

    pub fn sort_accounts(&mut self, direction: &str) {
        // defaults to ascneding sort in case of argument mistype
        match direction {
            "asc" => self.accounts.sort_by(|a, b| a.cmp(b)),
            "desc" => self.accounts.sort_by(|a, b| b.cmp(a)),
            _ => self.accounts.sort_by(|a, b| a.cmp(b))
        };        
    }

}

// Accounts are entities that have Transactions
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub id: String,
    pub subaccounts: Vec<String>,
    pub name: String,
    pub r#type: String,
    pub transactions: Vec<String>,
    pub parent: String

}

impl Account {

    pub fn cmp(&self, another: &Account) -> Ordering{

        let first = self.id.parse::<usize>().expect("Bad id ");
        let second = another.id.parse::<usize>().expect("Bad id ");

        return first.cmp(&second)
    }

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
