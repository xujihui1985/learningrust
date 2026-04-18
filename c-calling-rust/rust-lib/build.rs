use std::{
    env, fs,
    path::{Path, PathBuf},
};

fn main() {
    let crate_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is set"));
    let profile = env::var("PROFILE").expect("PROFILE is set");
    let target_dir = cargo_target_dir(&crate_dir);
    let header_dir = target_dir.join(profile).join("include");
    let header_path = header_dir.join("rust_lib.h");
    let config_path = crate_dir.join("cbindgen.toml");

    fs::create_dir_all(&header_dir).expect("create include directory");

    let config = cbindgen::Config::from_file(&config_path).expect("read cbindgen config");

    cbindgen::Builder::new()
        .with_crate(&crate_dir)
        .with_config(config)
        .generate()
        .expect("generate C header")
        .write_to_file(header_path);

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/calculator.rs");
    println!("cargo:rerun-if-changed=src/ffi.rs");
    println!("cargo:rerun-if-changed=cbindgen.toml");
}

fn cargo_target_dir(crate_dir: &Path) -> PathBuf {
    env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| crate_dir.join("target"))
}
