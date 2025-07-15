use borsh::{BorshDeserialize, BorshSerialize};
use risc0_zkvm::Receipt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct SimpleVerificationCircuitInput {
    pub receipt: Receipt,
}
