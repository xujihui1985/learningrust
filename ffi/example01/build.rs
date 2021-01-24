use std::env;
use cbindgen;

fn main() {
  let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

  cbindgen::generate(crate_dir)
    .expect("unable to generate bindings")
    .write_to_file("./include/header.h");
}
