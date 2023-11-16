fn main() {
    // std::env::set_var("CC", "/opt/homebrew/bin/gcc-12");
    // std::env::set_var("CXX", "/opt/homebrew/bin/g++-12");

    cxx_build::bridge("src/bridge.rs") // returns a cc::Build
        .cpp(true)
        .file("src/algorithms/cpp/floyd_warshall.cc")
        .flag_if_supported("-std=c++20")
        .compile("framework-rs");
}
