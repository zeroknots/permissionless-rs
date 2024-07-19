use crate::types::PackedUserOperation;
use crate::types::UserOpBuilder;
use ethereum_types::{Address, H256, U256};
use ethers::types::Bytes;

pub struct Safe7579;

impl UserOpBuilder for Safe7579 {
    fn new(
        &self,
        validator_address: Address,
        sender: Address,
        call_data: Bytes,
    ) -> PackedUserOperation {
        let default_h256 = H256::zero();
        let default_bytes = Bytes::default();

        let next_nonce = U256::zero();

        PackedUserOperation {
            sender,
            nonce: self.get_nonce(&next_nonce, validator_address),
            init_code: default_bytes.clone(),
            call_data,
            account_gas_limits: default_h256,
            pre_verification_gas: U256::zero(),
            gas_fees: default_h256,
            paymaster_and_data: default_bytes.clone(),
            signature: default_bytes,
        }
    }

    fn get_nonce(&self, nonce: &U256, validator_address: Address) -> ethereum_types::U256 {
        U256::zero()
    }
}
