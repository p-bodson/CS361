use std::cmp::Ordering;
use crate::company::Company;
use crate::transaction::Transaction;

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

        let first = self.id.parse::<usize>().expect("Bad id");
        let second = another.id.parse::<usize>().expect("Bad id");

        return first.cmp(&second)
    }

    pub fn new() -> Self {
        Account {
            id: "".to_string(),
            subaccounts: Vec::new(),
            name: "".to_string(),
            r#type: "".to_string(),
            transactions: Vec::new(),
            parent: "".to_string()
        }
    }

    pub fn set_id_in_company(&mut self, company: &mut Company) -> &mut Self {
        // set the id to be the next highest one in the company
        let current_max = company.max_id_account().unwrap();
        self.id = (current_max + 1).to_string();

        self
    }

    pub fn set_name(&mut self, name: &str) -> &mut Self {
        // sets the name of the account
        self.name = name.to_string();

        self
    }

    pub fn set_parent(&mut self, parent: &str) -> &mut Self {
        // sets the name of the account
        self.parent = parent.to_string();

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

    pub fn add_subaccount(&mut self, subaccount_id: &str) -> &mut Self {
        // adds the subaccount to the account if not already included
        let mut is_present = false;

        for account_id in self.subaccounts.iter() {
            if account_id == subaccount_id {
                is_present = true;
            }
        }

        if !is_present {
            self.subaccounts.push(subaccount_id.to_string());
        }

        self
    }

    pub fn remove_subaccount(&mut self, subaccount_id: &str) -> &mut Self {

        let mut idx_to_remove = 0;
        let mut idx_found = false;
        for (idx, account_id) in self.subaccounts.iter().enumerate() {
            if subaccount_id == account_id {
               idx_to_remove = idx;
               idx_found = true;
            }
        }

        if idx_found {
            self.subaccounts.remove(idx_to_remove);
        }

        self
    }

    pub fn add_transaction(&mut self, transaction_id: &str) -> &mut Self {
        // adds the transaction to the account if not already included
        let mut is_present = false;

        for id in self.transactions.iter() {
            if id == transaction_id {
                is_present = true;
            }
        }

        if !is_present {
            self.transactions.push(transaction_id.to_string());
        }

        self
    }

    pub fn remove_transaction(&mut self, transaction_id: &str) -> &mut Self {

        let mut idx_to_remove = 0;
        let mut idx_found = false;
        for (idx, id) in self.transactions.iter().enumerate() {
            if transaction_id == id {
               idx_to_remove = idx;
               idx_found = true;
            }
        }

        if idx_found {
            self.transactions.remove(idx_to_remove);
        }

        self
    }

}