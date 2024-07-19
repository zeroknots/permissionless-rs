
use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use ethereum_types::{Address, U256, H256 };
use ethers::types::Bytes;
use ethers::prelude::*;
use crate::types::Transaction;


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

impl SmartAccount {
    pub fn modules(&self) -> &InstalledModules {
        &self.modules
    }
}

pub trait ERC7579Account {
    fn execute( tx: Vec<Transaction>) -> Result<Bytes, Box<dyn std::error::Error>>;
    fn install_module( module: Address, module_type: ModuleType, init_params:Bytes) -> Result<Bytes, Box<dyn std::error::Error>>;
    fn uninstall_module( module: Address, module_type: ModuleType, init_params:Bytes) -> Result<Bytes, Box<dyn std::error::Error>>;
}


// pub fn execute<T: ERC7579Account> (tx: Vec<Transaction>) -> Result<Bytes, Box<dyn std::error::Error>> {
//     let len = tx.len();
//     // match len {
//     //     0 => Err("No transactions to execute".into()),
//     //     1 => T::execute(tx),
//     //     _ => {
//     //         Ok(result)
//     //     }
//     // }
// }

pub fn install_module<T: ERC7579Account> (module: Address, module_type: ModuleType, init_params:Bytes) -> Result<Bytes, Box<dyn std::error::Error>> {
    T::install_module(module, module_type, init_params)
}

pub fn uninstall_module<T: ERC7579Account> (module: Address, module_type: ModuleType, init_params:Bytes) -> Result<Bytes, Box<dyn std::error::Error>> {
    T::uninstall_module(module, module_type, init_params)
}
