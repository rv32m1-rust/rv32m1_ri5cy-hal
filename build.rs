use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const MISSING_CARGO_ENV: &'static str = "Missing environment variables provided by Cargo.";

/// Put the linker script somewhere the linker can find it.
fn put_memory_x(out_dir: &str) {
    let dest_path = Path::new(&out_dir);
    let mut f = File::create(&dest_path.join("memory.x")).expect("create file");
    f.write_all(include_bytes!("memory.x")).expect("write file");
    println!("cargo:rustc-link-search={}", dest_path.display());
    println!("cargo:rerun-if-changed=memory.x");
}

/// Build script for the crate.
fn main() {
    let out_dir = env::var("OUT_DIR").expect(MISSING_CARGO_ENV);
    put_memory_x(&out_dir);
    println!("cargo:rerun-if-changed=build.rs");
}
