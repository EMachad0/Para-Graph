#include "framework-rs/include/floyd_warshall.h"

void floyd_warshall(const size_t n, rust::slice<double> mat) {
    double res[n * n];
    for (size_t i = 0; i < n * n; ++i) {
        res[i] = mat[i];
    }
            
    #pragma acc region
    for (size_t k = 0; k < n; ++k) {
        #pragma acc loop
        for (size_t i = 0; i < n; ++i) {
            #pragma acc loop
            for (size_t j = 0; j < n; ++j) {
                double ik = res[i * n + k];
                double kj = res[k * n + j];
                double ij = res[i * n + j];
                res[i * n + j] = std::min(ij, ik + kj);
            }
        }
    }

    for (size_t i = 0; i < n * n; ++i) {
        mat[i] = res[i];
    }
}
