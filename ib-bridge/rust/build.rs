use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let _out_dir = PathBuf::from(env::var("OUT_DIR").unwrap()); // Unused but kept for future use
    let include_dir = PathBuf::from(&crate_dir).join("include"); // Use a reference here to avoid moving crate_dir

    // Create the include directory if it doesn't exist
    std::fs::create_dir_all(&include_dir).unwrap();

    // Generate header using cbindgen
    let config = cbindgen::Config::from_file("cbindgen.toml").unwrap_or_default();
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(config)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(include_dir.join("ib_bridge.h"));

    // Make cargo rerun this build script if cbindgen.toml changes
    println!("cargo:rerun-if-changed=cbindgen.toml");
    println!("cargo:rerun-if-changed=src/");
}
