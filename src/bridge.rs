#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("framework-rs/include/floyd_warshall.h");

        #[allow(dead_code)]
        fn floyd_warshall(mat: Vec<f64>) -> Vec<f64>;
    }
}
