use chrono::prelude::*;
use std::cmp::Ordering;
use crate::company::Company;


// Transactions move money between accounts
#[derive(Debug, Serialize, Deserialize, Clone)]
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

    pub fn new() -> Self {
        Transaction {
            id: "".to_string(),
            credit: "".to_string(),
            debit: "".to_string(),
            amount: "".to_string(),
            memo: "".to_string(),
            date: Utc::today().naive_utc()
        }
    }

    pub fn cmp(&self, another: &Transaction) -> Ordering {

        let first = self.id.parse::<usize>().expect("Bad id");
        let second = another.id.parse::<usize>().expect("Bad id");

        return first.cmp(&second)
    }

    pub fn set_id_in_company(&mut self, company: &mut Company) -> &mut Self {
        // set the id to be the next highest one in the company
        let current_max = company.max_id_transaction().unwrap();
        self.id = (current_max + 1).to_string();

        self
    }

    pub fn set_credit(&mut self, credit: &str) -> &mut Self {
        // sets the credit of the transaction
        self.credit = credit.to_string();

        self
    }

    pub fn set_debit(&mut self, debit: &str) -> &mut Self {
        // sets the debit of the transaction
        self.debit = debit.to_string();

        self
    }

    pub fn set_memo(&mut self, memo: &str) -> &mut Self {
        // sets the memo of the transaction
        self.memo = memo.to_string();

        self
    }

    pub fn set_amount(&mut self, amount: &str) -> &mut Self {
        // sets the memo of the transaction
        self.amount = amount.to_string();

        self
    }


}