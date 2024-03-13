fn main() {
    // std::env::set_var("CC", "/opt/homebrew/bin/gcc-13");
    // std::env::set_var("CXX", "/opt/homebrew/bin/g++-13");

    cxx_build::bridge("src/bridge.rs") // returns a cc::Build
        .cpp(true)
        .file("src/algorithms/cpp/floyd_warshall.cc")
        .flag_if_supported("-std=c++20")
        .compile("framework-rs");
    
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/algorithms/cpp/floyd_warshall.cc");
    println!("cargo:rerun-if-changed=include/floyd_warshall.h");
}
