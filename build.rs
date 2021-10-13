// build.rs
fn main() {
    cxx_build::bridge("src/main.rs") // returns a cc::Build
        .file("src/test.cc")
        .flag_if_supported("-std=c++11")
        .compile("rustcppinterop");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/test.cc");
    println!("cargo:rerun-if-changed=include/test.h");
}
