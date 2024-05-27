use std::collections::HashMap;

use anyhow::{anyhow, Result};

use crate::resp::parser::RespType;


#[derive(Debug)]
pub struct Danis {
    database: HashMap<String, RespType>,
}

impl Danis {
    pub fn new() -> Self {
        Self {
            database: HashMap::new()
        }
    }
    pub fn get(&mut self, key: String) -> Result<RespType> {
        match self.database.get(&key) {
            None => Err(anyhow!("Key is invalid.")),
            Some(value) => Ok(value.clone())
        }
    }

    pub fn set(&mut self, key: String, value: RespType) -> Result<()> {
        self.database.insert(key, value);

        Ok(())
    }
}