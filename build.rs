extern crate bindgen;

use std::fs;
use std::path::PathBuf;

fn main() {
    // Tell cargo to rerun build if any of the included headers change
    println!("cargo:rerun-if-changed=wrapper.h");

    // Use bindgen to generate the bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I/usr/include/freetype2")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("src/bindings");
    fs::create_dir_all(&out_path).expect("Couldn't create bindings directory");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
