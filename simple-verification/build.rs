use risc0_binfmt::compute_image_id;
use risc0_build::{embed_methods_with_options, DockerOptionsBuilder, GuestOptionsBuilder};
use std::{collections::HashMap, env, fs, path::Path};

fn main() {
    // Build environment variables
    println!("cargo:rerun-if-env-changed=SKIP_GUEST_BUILD");
    println!("cargo:rerun-if-env-changed=REPR_GUEST_BUILD");
    println!("cargo:rerun-if-env-changed=OUT_DIR");

    // Compile time constant environment variables
    println!("cargo:rerun-if-env-changed=TEST_SKIP_GUEST_BUILD");

    if std::env::var("CLIPPY_ARGS").is_ok() {
        let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
        let dummy_path = Path::new(&out_dir).join("methods.rs");
        fs::write(dummy_path, "// dummy methods.rs for Clippy\n")
            .expect("Failed to write dummy methods.rs");
        println!("cargo:warning=Skipping guest build in Clippy");
        return;
    }

    // Check if we should skip the guest build for tests
    if let Ok("1" | "true") = env::var("TEST_SKIP_GUEST_BUILD").as_deref() {
        println!("cargo:warning=Skipping guest build in test. Exiting");
        return;
    }

    let is_repr_guest_build = match env::var("REPR_GUEST_BUILD") {
        Ok(value) => match value.as_str() {
            "1" | "true" => {
                println!("cargo:warning=REPR_GUEST_BUILD is set to true");
                true
            }
            "0" | "false" => {
                println!("cargo:warning=REPR_GUEST_BUILD is set to false");
                false
            }
            _ => {
                println!("cargo:warning=Invalid value for REPR_GUEST_BUILD: '{}'. Expected '0', '1', 'true', or 'false'. Defaulting to false.", value);
                false
            }
        },
        Err(env::VarError::NotPresent) => {
            println!("cargo:warning=REPR_GUEST_BUILD not set. Defaulting to false.");
            false
        }
        Err(env::VarError::NotUnicode(_)) => {
            println!(
                "cargo:warning=REPR_GUEST_BUILD contains invalid Unicode. Defaulting to false."
            );
            false
        }
    };

    // Use embed_methods_with_options with our custom options
    let guest_pkg_to_options = get_guest_options();
    embed_methods_with_options(guest_pkg_to_options);

    // After the build is complete, copy the generated file to the elfs folder
    if is_repr_guest_build {
        println!("cargo:warning=Copying binary to elfs folder");
        copy_binary_to_elfs_folder();
    } else {
        println!("cargo:warning=Not copying binary to elfs folder");
    }
}

fn get_guest_options() -> HashMap<&'static str, risc0_build::GuestOptions> {
    let mut guest_pkg_to_options = HashMap::new();

    let opts = if env::var("REPR_GUEST_BUILD").is_ok() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let root_dir = format!("{manifest_dir}/../../");

        println!(
            "cargo:warning=Using Docker for guest build with root dir: {}",
            root_dir
        );

        let docker_opts = DockerOptionsBuilder::default()
            .root_dir(root_dir)
            .build()
            .unwrap();

        GuestOptionsBuilder::default()
            // .features(features)
            .use_docker(docker_opts)
            .build()
            .unwrap()
    } else {
        println!("cargo:warning=Guest code is not built in docker");
        GuestOptionsBuilder::default()
            // .features(features)
            .build()
            .unwrap()
    };

    guest_pkg_to_options.insert("simple-verification-guest", opts);
    guest_pkg_to_options
}

fn copy_binary_to_elfs_folder() {
    let current_dir = env::current_dir().expect("Failed to get current dir");
    let base_dir = current_dir.join("../");

    // Create elfs directory if it doesn't exist
    let elfs_dir = base_dir.join("elfs");
    if !elfs_dir.exists() {
        fs::create_dir_all(&elfs_dir).expect("Failed to create elfs directory");
        println!("cargo:warning=Created elfs directory at {:?}", elfs_dir);
    }

    // Build source path
    let src_path = current_dir.join("target/riscv-guest/simple-verification/simple-verification-guest/riscv32im-risc0-zkvm-elf/docker/simple-verification-guest.bin");
    if !src_path.exists() {
        println!(
            "cargo:warning=Source binary not found at {:?}, skipping copy",
            src_path
        );
        return;
    }

    let dest_filename = format!("simple-verification-guest.bin");
    let dest_path = elfs_dir.join(&dest_filename);

    // Copy the file
    match fs::copy(&src_path, &dest_path) {
        Ok(_) => println!(
            "cargo:warning=Successfully copied binary to {:?}",
            dest_path
        ),
        Err(e) => println!("cargo:warning=Failed to copy binary: {}", e),
    }

    let elf_path = "../elfs/simple-verification-guest.bin";

    let elf_bytes: Vec<u8> = fs::read(Path::new(elf_path)).expect("Failed to read ELF file");

    let method_id = compute_image_id(elf_bytes.as_slice()).unwrap();
    println!("cargo:warning=Computed method ID: {:x?}", method_id);
    println!(
        "cargo:warning=Computed method ID words: {:?}",
        method_id.as_words()
    );
}
