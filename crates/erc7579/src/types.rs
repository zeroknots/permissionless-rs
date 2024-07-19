
use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use ethereum_types::{Address, U256, H256 };
use ethers::types::Bytes;
use ethers::prelude::*;
use crate::execution_lib::ExecutionLib;
use tx_builder::Execution;


type ModuleType = H256;

const VALIDATOR_MODULE_TYPE: ModuleType = H256([1u8; 32]);
const EXECUTOR_MODULE_TYPE: ModuleType = H256([2u8; 32]);
const FALLBACK_MODULE_TYPE: ModuleType = H256([3u8; 32]);
const HOOK_MODULE_TYPE: ModuleType = H256([4u8; 32]);

#[derive(Serialize, Deserialize, Debug, Clone)]
struct InstalledModules {
    pub address: Address,
    pub module_types: Vec<ModuleType>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SmartAccount {
    pub address: Address,
    pub balance: U256,
    pub modules: InstalledModules,
}


pub trait ERC7579Account {
    fn execute( tx: Vec<Transaction>) -> Result<Bytes, Box<dyn std::error::Error>>;
    fn install_module( module: Address, module_type: ModuleType, init_params:Bytes) -> Result<Bytes, Box<dyn std::error::Error>>;
    fn uninstall_module( module: Address, module_type: ModuleType, init_params:Bytes) -> Result<Bytes, Box<dyn std::error::Error>>;
}



pub fn execute(tx: Vec<Execution>) -> Result<Bytes, Box<dyn std::error::Error>> {
    match tx.len() {
        0 => Err("No transactions to execute".into()),
        1 => Ok(Bytes::from(ExecutionLib::encode_single(
            tx[0].target,
            tx[0].value,
            tx[0].call_data.clone()
        ))),
        _ => Ok(Bytes::from(ExecutionLib::encode_batch(tx)))
    }
}

pub fn install_module<T: ERC7579Account> (module: Address, module_type: ModuleType, init_params:Bytes) -> Result<Bytes, Box<dyn std::error::Error>> {
    T::install_module(module, module_type, init_params)
}

pub fn uninstall_module<T: ERC7579Account> (module: Address, module_type: ModuleType, init_params:Bytes) -> Result<Bytes, Box<dyn std::error::Error>> {
    T::uninstall_module(module, module_type, init_params)
}
