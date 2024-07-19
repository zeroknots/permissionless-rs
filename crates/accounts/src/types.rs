use ethereum_types::{Address, H256, U256};
use ethers::prelude::*;
use ethers::prelude::*;
use ethers::types::Bytes;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

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

pub trait UserOpBuilder {
    fn new(
        &self,
        account_address: Address,
        validator_address: Address,
        call_data: Bytes,
    ) -> PackedUserOperation;
    fn get_nonce(&self, nonce: &U256, validator_address: Address) -> U256;
}
