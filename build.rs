fn main() {
    cxx_build::bridge("src/main.rs")
        .file("cpp/test.cpp")
        .compile("cxx-example");

    println!("cargo:rustc-link-lib=c");
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=cpp/test.cpp");
    println!("cargo:rerun-if-changed=cpp/include/test.hpp");
}
