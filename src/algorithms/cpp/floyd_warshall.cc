#include "framework-rs/include/floyd_warshall.h"

void floyd_warshall(const size_t n, rust::slice<double> mat) {
    for (size_t k = 0; k < n; ++k) {
        for (size_t i = 0; i < n; ++i) {
            for (size_t j = 0; j < n; ++j) {
                double ik = mat[i * n + k];
                double kj = mat[k * n + j];
                double ij = mat[i * n + j];
                mat[i * n + j] = std::min(ij, ik + kj);
            }
        }
    }
}
