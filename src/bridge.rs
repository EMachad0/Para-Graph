#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("framework-rs/include/floyd_warshall.h");

        #[allow(dead_code)]
        unsafe fn floyd_warshall(n: usize, mat: &mut [f64]);
    }
}
