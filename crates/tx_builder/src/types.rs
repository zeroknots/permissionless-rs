use clap::Parser;
use ethereum_types::{Address, H256, U256};
use ethers::types::Bytes;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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
    pub transactions: Vec<Execution>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    pub name: String,
    pub description: String,
    #[serde(rename = "txBuilderVersion")]
    pub tx_builder_version: String,
    #[serde(rename = "accountAddress")]
    pub account_address: Option<Address>,
    #[serde(rename = "accountType")]
    pub account_type: String,
    #[serde(rename = "validatorModule")]
    pub validator_module: Address,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Execution {
    pub target: Address,
    pub value: U256,
    pub call_data: Option<Bytes>,
}

// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct Execution {
//     pub target: Address,
//     pub value: U256,
//     pub call_data: Option<Bytes>,
// }
