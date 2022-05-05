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