fn main() {
    std::env::set_var(
        "CC",
        "/opt/nvidia/hpc_sdk/Linux_x86_64/24.3/compilers/bin/nvc",
    );
    std::env::set_var(
        "CXX",
        "/opt/nvidia/hpc_sdk/Linux_x86_64/24.3/compilers/bin/nvc++",
    );

    // disable default flags
    std::env::set_var("CRATE_CC_NO_DEFAULTS", "1");

    cxx_build::bridge("src/bridge.rs") // returns a cc::Build
        .cpp(true)
        .file("src/algorithms/cpp/floyd_warshall.cc")
        .flag_if_supported("-std=c++17")
        .flag("-fPIC")
        .flag("-acc")
        .flag("-gpu=nordc") // device link at compile time
        // .flag("-ta=tesla:nordc") // same as above but deprecated
        .flag("-Minfo=accel")
        .compile("framework-rs");

    // openacc libs
    // bottom loads first
    println!("cargo:rustc-link-lib=dylib=acchost");
    println!("cargo:rustc-link-lib=dylib=accdevice");
    println!("cargo:rustc-link-lib=dylib=nvhpcman");
    println!("cargo:rustc-link-lib=dylib=dl");
    println!("cargo:rustc-link-lib=dylib=cudadevice");
    println!("cargo:rustc-link-lib=dylib=atomic");
    println!("cargo:rustc-link-lib=dylib=nvhpcatm");
    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rustc-link-lib=dylib=nvomp");
    println!("cargo:rustc-link-lib=dylib=dl");
    println!("cargo:rustc-link-lib=dylib=nvhpcatm");
    println!("cargo:rustc-link-lib=dylib=atomic");
    println!("cargo:rustc-link-lib=dylib=pthread");
    println!("cargo:rustc-link-lib=dylib=nvcpumath");
    println!("cargo:rustc-link-lib=dylib=nsnvc");
    println!("cargo:rustc-link-lib=dylib=nvc");
    println!("cargo:rustc-link-lib=dylib=m");
    println!("cargo:rustc-link-lib=dylib=gcc");
    println!("cargo:rustc-link-lib=dylib=c");
    println!("cargo:rustc-link-lib=dylib=gcc");
    println!("cargo:rustc-link-lib=dylib=gcc_s");
    println!("cargo:rustc-link-lib=dylib=nvcpumath");
    println!("cargo:rustc-link-lib=dylib=acccuda");
    println!("cargo:rustc-link-lib=dylib=accdevaux");

    // hot reload
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/algorithms/cpp/floyd_warshall.cc");
    println!("cargo:rerun-if-changed=include/floyd_warshall.h");
}
