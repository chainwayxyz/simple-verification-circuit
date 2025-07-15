pub mod structs;
use risc0_zkvm::{guest::env, sha::Digestible};
use structs::SimpleVerificationCircuitInput;

use crate::common::zkvm::ZkvmGuest;

pub fn simple_verification_circuit(guest: &impl ZkvmGuest) {
    let input: SimpleVerificationCircuitInput = guest.read_from_host();
    let method_id = input
        .receipt
        .claim()
        .expect("Receipt must have a claim")
        .value()
        .expect("Claim value must be present")
        .pre
        .value()
        .expect("Pre value must be present")
        .digest();
    let journal_digest = input.receipt.journal.bytes;

    env::verify(method_id, &journal_digest).unwrap();

    let mut concat_output = method_id.as_bytes().to_vec();
    concat_output.extend_from_slice(&journal_digest);

    let final_hash = blake3::hash(&concat_output);

    guest.commit(&final_hash.as_bytes());
}
