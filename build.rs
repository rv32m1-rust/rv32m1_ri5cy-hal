// Ref: Sha Miao, rv32m1_ri5cy-example/build.rs

use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

const MISSING_CARGO_ENV: &'static str = "Missing environment variables provided by Cargo.";

/// Put the linker script somewhere the linker can find it.
fn put_memory_x(out_dir: &str) {
    let dest_path = Path::new(&out_dir);
    let mut f = File::create(&dest_path.join("memory.x")).expect("Could not create file");
    f.write_all(include_bytes!("memory.x"))
        .expect("Could not write file");
    println!("cargo:rustc-link-search={}", dest_path.display());
    println!("cargo:rerun-if-changed=memory.x");
}

/// Include `.a` files generated by assembly codes
fn include_a_files(out_dir: &str) {
    let target = env::var("TARGET").expect(MISSING_CARGO_ENV);
    let out_dir = PathBuf::from(out_dir);
    let name = env::var("CARGO_PKG_NAME").expect(MISSING_CARGO_ENV);

    if &target == "riscv32imc-unknown-none-elf" {
        fs::copy(
            format!("bin/{}.a", target),
            out_dir.join(format!("lib{}.a", name)),
        )
        .unwrap();

        println!("cargo:rustc-link-lib=static={}", name);
        println!("cargo:rustc-link-search={}", out_dir.display());
        println!("cargo:rerun-if-changed=bin/{}.a", target);
    }
}

/// Build script for the crate.
fn main() {
    let out_dir = env::var("OUT_DIR").expect(MISSING_CARGO_ENV);
    put_memory_x(&out_dir);
    include_a_files(&out_dir);
    println!("cargo:rerun-if-changed=build.rs");
}
