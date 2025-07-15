# Simple Verification Circuit

A RISC0 zkVM-based project for zero-knowledge proof generation and verification.

## Project Structure

- `circuits-lib/` - Core library containing circuit implementations
- `simple-verification/` - Guest program that runs inside the zkVM
- `simple-verification-host/` - Host program that generates proofs
- `elfs/` - Directory containing compiled guest binaries

## Building the Project

### 1. Build the Guest ELF

First, navigate to the `simple-verification` directory and build the guest program:

```bash
cd simple-verification
REPR_GUEST_BUILD=1 cargo build -p simple-verification --release
```

This will generate the ELF file that will be executed inside the zkVM.

## Usage

### Generating Proofs

1. **Update the receipt file path**: In the host program (`simple-verification-host/src/main.rs`), modify the `include_bytes!` path to point to your desired receipt file:

```rust
let receipt = include_bytes!("../receipts/your_receipt_file.bin");
```

2. **Run the host program** to generate the proof:

```bash
cargo run --package simple-verification-host --bin simple-verification-host
```

This will:
- Load the specified receipt file
- Execute the guest program in the zkVM
- Generate a zero-knowledge proof of correct execution
- Output the verification results
