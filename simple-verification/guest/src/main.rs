use circuits_lib::common;
use circuits_lib::simple_verification::simple_verification_circuit;

fn main() {
    let zkvm_guest = common::zkvm::Risc0Guest::new();
    simple_verification_circuit(&zkvm_guest);
}
