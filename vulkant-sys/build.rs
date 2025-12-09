use std::{env, path};

fn main() {
    println!("cargo:rustc-link-lib=vulkan");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .use_core()
        .wrap_unsafe_ops(true)
        .derive_default(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out = path::PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out.join("bindings.rs")).expect("Unable to write bindings");
}
