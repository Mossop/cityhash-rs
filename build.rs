extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn build(target: &str, functions: &[&str]) {
    let mut builder = bindgen::Builder::default()
        .header(format!("{}/city.h", target));

    for function in functions {
        builder = builder.whitelist_function(function);
    }

    let bindings = builder
        .clang_args(&["-x", "c++"])
        .generate()
        .expect("Unable to generate bindings.");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join(format!("{}.rs", target)))
        .expect("Couldn't write bindings!");

    cc::Build::new()
        .file(format!("{}/city.cc", target))
        .include(format!("{}", target))
        .cpp(true)
        .compile(target);
}

fn main() {
    build("cityhash_1", &[
        "CityHash64_1",
        "CityHash128_1",
    ]);
    build("cityhash_1_1_1", &[
        "CityHash64_1_1_1",
        "CityHash128_1_1_1",
    ]);
}
