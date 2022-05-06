use chrono::prelude::*;
use std::error::Error;

use std::cmp::Ordering;

use crate::file_io;
use crate::account::Account;
use crate::transaction::Transaction;

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
        // will not insert account with non-unique id

        let mut is_unique = true; 

        for element in &self.accounts {
            if element.id == account.id {
                is_unique = false;
            }
        }

        if is_unique {
            self.accounts.push(account);
        }
    }

    pub fn delete_account(&mut self, to_delete: &str) {

        let mut idx_to_delete = 0;
        let mut idx_found = false;

        for (idx, account) in self.accounts.iter().enumerate() {
            if to_delete == account.id {
                idx_to_delete = idx;
                idx_found = true;
            }
        }

        if idx_found {
            self.accounts.remove(idx_to_delete);
        }
    }

    pub fn insert_transaction(&mut self, transaction: Transaction) {
        // will not insert transaction with non-unique id

        let mut is_unique = true; 

        for element in &self.transactions {
            if element.id == transaction.id {
                is_unique = false;
            }
        }

        if is_unique {
            self.transactions.push(transaction);
        }
    }

    pub fn delete_transaction(&mut self, to_delete: &str) {

        let mut idx_to_delete = 0;
        let mut idx_found = false;

        for (idx, transaction) in self.transactions.iter().enumerate() {
            if to_delete == transaction.id {
                idx_to_delete = idx;
                idx_found = true;
            }
        }

        if idx_found {
            self.transactions.remove(idx_to_delete);
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

    pub fn sort_transactions(&mut self, direction: &str) {
        // defaults to ascneding sort in case of argument mistype
        match direction {
            "asc" => self.transactions.sort_by(|a, b| a.cmp(b)),
            "desc" => self.transactions.sort_by(|a, b| b.cmp(a)),
            _ => self.transactions.sort_by(|a, b| a.cmp(b))
        };        
    }

    pub fn max_id_account(&mut self) -> Result<usize, Box<dyn Error>> {
        // find the maximum id in the accounts
        self.sort_accounts("desc");
        let max_id = self.accounts[0].id.parse::<usize>()?;
        Ok(max_id)
    }

    pub fn max_id_transaction(&mut self) -> Result<usize, Box<dyn Error>> {
        // find the maximum id in the transactions
        self.sort_transactions("desc");
        let max_id = self.transactions[0].id.parse::<usize>()?;
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
