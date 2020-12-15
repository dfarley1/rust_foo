extern crate bindgen;
extern crate cbindgen;

use std::{env, path::Path, path::PathBuf};

const HEADER_ROOT: &str = "..";
const HEADERS: &[&str] = &["foo.h"];

// Enable calling C code from Rust
fn do_bindgen() {
    let mut bindings = bindgen::Builder::default()
        .generate_inline_functions(true)
        .generate_comments(false);
    for h in HEADERS {
        let header_path = Path::new(HEADER_ROOT).join(h);
        bindings = bindings.header(header_path.to_str().unwrap());

        let mut inc_path = String::from("-I");
        inc_path.push_str(header_path.parent().unwrap().to_str().unwrap());
        bindings = bindings.clang_arg(inc_path);
    }

    let generated_bindings = bindings
        .generate()
        .expect("Failed to generate bindgen bindings");
    let out_path = Path::new(&env::var("OUT_DIR").unwrap()).join("rust_bindgen.rs");
    generated_bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindgen bindings!");
}

// Enable calling Rust code from C
fn do_cbindgen() {
    let config =
        cbindgen::Config::from_file("./cbindgen.toml").expect("Failed to read cbindgen.toml");
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let bindings = cbindgen::generate_with_config(crate_dir, config)
        .expect("Failed to generate cbindgen bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let release_path = out_path
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    bindings.write_to_file(release_path.join("rust_cbindgen.h"));
}

fn main() {
    do_bindgen();
    do_cbindgen();
}
