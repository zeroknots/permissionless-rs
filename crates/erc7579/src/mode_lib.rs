use ethers::types::H256;
use ethers::utils::keccak256;

// Custom types
type ModeCode = H256;
type CallType = u8;
type ExecType = u8;
type ModeSelector = [u8; 4];
type ModePayload = [u8; 22];

// Constants
const CALLTYPE_SINGLE: CallType = 0x00;
const CALLTYPE_BATCH: CallType = 0x01;
const CALLTYPE_STATIC: CallType = 0xFE;
const CALLTYPE_DELEGATECALL: CallType = 0xFF;

const EXECTYPE_DEFAULT: ExecType = 0x00;
const EXECTYPE_TRY: ExecType = 0x01;

const MODE_DEFAULT: ModeSelector = [0x00; 4];
const MODE_OFFSET: ModeSelector = [0x01; 4]; // TODO

pub struct ModeLib;

impl ModeLib {
    pub fn decode(mode: ModeCode) -> (CallType, ExecType, ModeSelector, ModePayload) {
        let bytes = mode.as_bytes();
        let call_type = bytes[0];
        let exec_type = bytes[1];
        let mode_selector: ModeSelector = bytes[6..10].try_into().unwrap();
        let mode_payload: ModePayload = bytes[10..32].try_into().unwrap();
        (call_type, exec_type, mode_selector, mode_payload)
    }

    pub fn encode(
        call_type: CallType,
        exec_type: ExecType,
        mode: ModeSelector,
        payload: ModePayload,
    ) -> ModeCode {
        let mut bytes = [0u8; 32];
        bytes[0] = call_type;
        bytes[1] = exec_type;
        bytes[6..10].copy_from_slice(&mode);
        bytes[10..32].copy_from_slice(&payload);
        H256::from(bytes)
    }

    pub fn encode_simple_batch() -> ModeCode {
        Self::encode(CALLTYPE_BATCH, EXECTYPE_DEFAULT, MODE_DEFAULT, [0u8; 22])
    }

    pub fn encode_simple_single() -> ModeCode {
        Self::encode(CALLTYPE_SINGLE, EXECTYPE_DEFAULT, MODE_DEFAULT, [0u8; 22])
    }

    pub fn get_call_type(mode: ModeCode) -> CallType {
        mode.as_bytes()[0]
    }
}

// Helper functions for comparisons
pub fn eq_call_type(a: CallType, b: CallType) -> bool {
    a == b
}

pub fn neq_call_type(a: CallType, b: CallType) -> bool {
    a != b
}

pub fn eq_exec_type(a: ExecType, b: ExecType) -> bool {
    a == b
}

pub fn eq_mode_selector(a: &ModeSelector, b: &ModeSelector) -> bool {
    a == b
}
