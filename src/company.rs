
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

    pub fn max_id_account(&mut self) -> Result<usize, Box<dyn Error>> {
        // find the maximum id in the accounts
        self.sort_accounts("desc");
        let max_id = self.accounts[0].id.parse::<usize>()?;
        Ok(max_id)
    }

    pub fn get_acccount_by_id(&self, id: &str) -> Option<&Account> {
        let mut account_index: usize = 0;
        let mut account_found = false;

        for (idx, account) in self.accounts.iter().enumerate() {
            if account.id == id {
                account_index = idx;
                account_found = true;
            }
        }

        if account_found {
            Some(&self.accounts[account_index])
        }
        else {
            None
        }
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

    pub fn cmp(&self, another: &Account) -> Ordering {

        let first = self.id.parse::<usize>().expect("Bad id ");
        let second = another.id.parse::<usize>().expect("Bad id ");

        return first.cmp(&second)
    }

    pub fn new() -> Self {
        Account {
            id: "0".to_string(),
            subaccounts: Vec::new(),
            name: "Unnamed".to_string(),
            r#type: "d".to_string(),
            transactions: Vec::new(),
            parent: "0".to_string()
        }
    }

    pub fn set_id_in_company(&mut self, company: &mut Company) -> &mut Self{
        // set the id to be the next highest one in the company
        let current_max = company.max_id_account().unwrap();
        self.id = (current_max + 1).to_string();

        self
    }

    pub fn set_type_in_company(&mut self, r#type: &str, company: &Company) -> &mut Self{
        // set type as parent if possible, otherwise set as requested

        let good_type = match r#type {
            "d" => true,
            "c" => true,
            _ => false
        };

        if !good_type {
            return self;
        }

        if self.parent == "0" {
            // at root
            self.r#type = r#type.to_string();
        }
        else {
            let result = company.get_acccount_by_id(&self.parent[..]);
            match result {
                Some(x) => {
                    println!("parent is {:?}", x);
                    self.r#type = x.r#type.to_string();
                },
                None => {
                    println!("no parent");
                    self.r#type = r#type.to_string();
                },
            }
        }

        self
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
