fn main() {
    //     const CXX_FLAGS: &str =
    //         "-ffreestanding -mno-red-zone -fshort-wchar -fno-exceptions -fno-rtti -std=c++17 -L/usr/lib/x86_64-linux-gnu";
    // const CXX_FLAGS: &str = "-std=c++17 -L/usr/lib/x86_64-linux-gnu";
    // const CXX_FLAGS: &str = "";
    let mut cc = cxx_build::bridge("src/main.rs");
    // let mut cc = &mut cc;
    // for flag in CXX_FLAGS.split_whitespace() {
    //     cc = cc.flag(flag);
    // }
    cc.file("cpp/test.cpp").compile("cxx-example");

    println!("cargo:rustc-link-lib=c");
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=cpp/test.cpp");
    println!("cargo:rerun-if-changed=cpp/include/test.hpp");
}
