use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum TraceEvent {
    InstructionFetched {
        pc: u64,
        opcode_byte: u8,
    },
    Decoded {
        pc: u64,
        mnemonic: String,
        approx_mode: String,
        fallback_policy: String,
        confidence_target: u8,
    },
    ContractEvaluated {
        opcode: String,
        confidence_achieved: f64,
        confidence_target: f64,
        target_met: bool,
    },
    FallbackTriggered {
        policy: String,
        reason: String,
    },
    Executed {
        result: u64,
        substrate: String,
    },
    ContractViolation {
        opcode: String,
        detail: String,
    },
}
