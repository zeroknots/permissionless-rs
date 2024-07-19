use crate::types::TransactionData;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub fn parse_tx_file(file_path: &str) -> Result<TransactionData, Box<dyn std::error::Error>> {
    println!("Parsing file: {}", file_path);
    // Open the file
    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            println!("Error opening file: {:?}", e);
            return Err(Box::new(e));
        }
    };
    println!("File opened");
    let reader = BufReader::new(file);
    
    // Parse the JSON
    let data: TransactionData = serde_json::from_reader(reader)?;
    
    Ok(data)
}
