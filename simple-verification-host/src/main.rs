// write a main function

use circuits_lib::simple_verification::structs::SimpleVerificationCircuitInput;
use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts};

pub const SIMPLE_VERIFICATION_CIRCUIT_ELF: &[u8] =
    include_bytes!("../../elfs/simple-verification-guest.bin");

fn main() {
    let receipt = include_bytes!("../receipts/header_chain_proof_100.bin");

    let receipt =
        borsh::from_slice::<risc0_zkvm::Receipt>(receipt).expect("Failed to deserialize receipt");

    println!("Receipt: {:?}", receipt);

    let simple_verification_circuit_input = SimpleVerificationCircuitInput {
        receipt: receipt.clone(),
    };

    let mut binding = ExecutorEnv::builder();
    let env = binding
        .write_slice(
            &borsh::to_vec(&simple_verification_circuit_input)
                .expect("Serialization to vec is infallible"),
        )
        .add_assumption(receipt)
        .build()
        .expect("Failed to build ExecutorEnv");

    let prover = default_prover();

    let succinct_receipt = prover
        .prove_with_opts(
            env,
            SIMPLE_VERIFICATION_CIRCUIT_ELF,
            &ProverOpts::succinct(),
        )
        .expect("Failed to prove")
        .receipt;

    println!("Proved successfully! Receipt: {:?}", succinct_receipt);
}
