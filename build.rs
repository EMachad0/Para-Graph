use std::process::Command;

fn main() {
    // Check if nvc++ is available
    let nvcxx_available = Command::new("nvc++").arg("--version").output().is_ok();

    if nvcxx_available {
        // Set environment variables for nvc++ compiler
        std::env::set_var("CXX", "nvc++");
    } else {
        // Fallback to default compiler, e.g., g++
        println!("cargo:warning=nvc++ compiler not found. Falling back to default C++ compiler.");
    }

    // disable default flags
    std::env::set_var("CRATE_CC_NO_DEFAULTS", "1");

    let mut build = cxx_build::bridge("src/bridge.rs"); // returns a cc::Build
    build
        .cpp(true) // use CXX compiler
        .file("src/algorithms/cpp/floyd_warshall.cc")
        .flag_if_supported("-std=c++17") // latest suported by openacc
        .flag("-fPIC"); // make as library, position independent code

    if nvcxx_available {
        build.flag("-acc").flag("-gpu=nordc"); // device link at compile time

        // Add "-Minfo=accel" flag only in development mode
        if std::env::var("PROFILE").unwrap_or_default() == "debug" {
            build.flag("-Minfo=accel");
        }
    }

    build.compile("para-graph");

    if nvcxx_available {
        // openacc libs
        // loads bottom up for some reason
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
    }

    // hot reload
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/algorithms/cpp/floyd_warshall.cc");
    println!("cargo:rerun-if-changed=include/floyd_warshall.h");
}
