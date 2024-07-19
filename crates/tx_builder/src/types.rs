use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use clap::Parser;
// use ethereum_types::{Address, U256, Bytes};


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub file: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionData {
    pub version: String,
    #[serde(rename = "chainId")]
    pub chain_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: u64,
    pub meta: Meta,
    pub transactions: Vec<Transaction>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    pub name: String,
    pub description: String,
    #[serde(rename = "txBuilderVersion")]
    pub tx_builder_version: String,
    #[serde(rename = "accountAddress")]
    pub account_address: String,
    #[serde(rename = "accountType")]
    pub account_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub to: String,
    pub value: String,
    pub data: Option<String>,
}

// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct Execution {
//     pub target: Address,
//     pub value: U256,
//     pub call_data: Option<Bytes>,
// }
