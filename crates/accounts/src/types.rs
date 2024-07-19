use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use ethereum_types::{Address, U256, H256, Bytes};
use ethers::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PackedUserOperation {
    pub sender: Address,
    pub nonce: U256,
    pub init_code: Bytes,
    pub call_data: Bytes,
    pub account_gas_limits: H256,
    pub pre_verification_gas: U256,
    pub gas_fees: H256,
    pub paymaster_and_data: Bytes,
    pub signature: Bytes,
}


pub trait UserOpBuilder{
    fn create_user_op(nonce: U256, validator_address: Address, account_address: Address, call_data:Bytes) -> PackedUserOperation;
    fn get_nonce(nonces: &[U256], validator_address:Address) -> U256;
}
