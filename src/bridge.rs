#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("para-graph/include/floyd_warshall.h");
        include!("para-graph/include/gaussian_elimination.h");

        unsafe fn floyd_warshall(n: usize, mat: &mut [f64]);

        unsafe fn gaussian_elimination(
            n: usize,
            mat: &[Vec<f64>],
        ) -> Result<UniquePtr<CxxVector<f64>>>;
    }
}
