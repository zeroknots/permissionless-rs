use ethers::types::{Address, Bytes, U256};
use tx_builder::Execution;

pub struct ExecutionLib;

impl ExecutionLib {
    pub fn decode_batch(call_data: &[u8]) -> Vec<Execution> {
        let data = &call_data[4..];
        let executions_count = U256::from_big_endian(&data[0..32]).as_usize();
        let mut executions = Vec::with_capacity(executions_count);
        
        let mut offset = 32;
        for _ in 0..executions_count {
            let target: Address = Address::from_slice(&data[offset..offset+20]);
            offset += 20;
            
            let value = U256::from_big_endian(&data[offset..offset+32]);
            offset += 32;
            
            let call_data_length = U256::from_big_endian(&data[offset..offset+32]).as_usize();
            offset += 32;
            
            let call_data = Bytes::from(data[offset..offset+call_data_length].to_vec());
            offset += call_data_length;
            
            executions.push(Execution {
                target,
                value,
                call_data: todo!(),
            });
        }
        
        executions
    }

    pub fn encode_batch(executions: Vec<Execution>) -> Bytes {
        // let mut encoded = Vec::new();
        // let mut buffer = [0u8; 32];
        //
        // U256::from(executions.len()).to_big_endian(&mut buffer);
        // encoded.extend_from_slice(&buffer);
        //
        // for execution in executions {
        //     encoded.extend_from_slice(execution.target.as_bytes());
        //     execution.value.to_big_endian(&mut buffer);
        //     encoded.extend_from_slice(&buffer);
        //     U256::from(execution.call_data.len()).to_big_endian(&mut buffer);
        //     encoded.extend_from_slice(&buffer);
        //     encoded.extend_from_slice(&execution.call_data);
        // }
        //
        // encoded.into()
        return Bytes::from(vec![1]);
    }

    pub fn decode_single(execution_calldata: &[u8]) -> (Address, U256, Bytes) {
        let target = Address::from_slice(&execution_calldata[0..20]);
        let value = U256::from_big_endian(&execution_calldata[20..52]);
        let call_data = Bytes::from(execution_calldata[52..].to_vec());
        
        (target, value, call_data)
    }

    pub fn encode_single(target: Address, value: U256, call_data: Option<Bytes>) -> Bytes{
        // let mut user_op_calldata = Vec::new();
        // let mut buffer = [0u8; 32];
        // user_op_calldata.extend_from_slice(target.as_bytes());
        // value.to_big_endian(&mut buffer);
        // user_op_calldata.extend_from_slice(&buffer);
        // user_op_calldata.extend_from_slice(call_data);
        // user_op_calldata
        return Bytes::from(vec![1]);
    }
}
