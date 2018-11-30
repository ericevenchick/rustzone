extern crate bindgen;

use std::env;
use std::path::PathBuf;

const OPTEE_PATH: &'static str = "/home/eric/devel/optee/";

fn main() {
    // Tell cargo to tell rustc to link the utee library and dependencies 
    println!("cargo:rustc-link-search={}optee_os/out/arm/export-ta_arm64/lib/", OPTEE_PATH);
    println!("cargo:rustc-link-lib=utee");
    println!("cargo:rustc-link-lib=mpa");
    println!("cargo:rustc-link-lib=util");

    let libutee_include = format!("-I{}{}", OPTEE_PATH, "optee_os/lib/libutee/include/");
    let libutils_include = format!("-I{}{}", OPTEE_PATH, "optee_os/lib/libutils/ext/include/");
    let linux_include = format!("-I{}{}", OPTEE_PATH, "linux/include/");
    let uapi_include = format!("-I{}{}", OPTEE_PATH, "linux/include/uapi/linux/");
    let compiler_include = format!("-I{}{}", OPTEE_PATH, "toolchains/aarch32/arm-linux-gnueabihf/libc/usr/include/");
    let stdlib_include = format!("-I{}{}", OPTEE_PATH, "toolchains/aarch32/lib/gcc/arm-linux-gnueabihf/8.2.1/include/");


    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // use hard float
        .clang_arg("-D__ARM_PCS_VFP")
        // Add args for paths to headers
        .clang_arg(libutee_include)
        .clang_arg(libutils_include)
        .clang_arg(linux_include)
        //.clang_arg(uapi_include)
        .clang_arg(compiler_include)
        .clang_arg(stdlib_include)
        // use core, no std
        .use_core()
        .ctypes_prefix("c_types")
        // ignore functions we'll define
        .hide_type("TA_.*")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
