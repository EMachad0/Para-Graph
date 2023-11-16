#include "framework-rs/include/floyd_warshall.h"
#include <vector>
#include <cmath>

void floyd_warshall_impl(double *mat, size_t n, size_t m) {
    for (size_t k = 0; k < n; ++k) {
        for (size_t i = 0; i < n; ++i) {
            for (size_t j = 0; j < m; ++j) {
                double ik = mat[i * n + k];
                double kj = mat[k * n + j];
                double ij = mat[i * n + j];
                mat[i * n + j] = std::min(ij, ik + kj);
            }
        }
    }
}

rust::vec<double> floyd_warshall(rust::vec<double> r_mat) {
    std::vector<double> mat(r_mat.begin(), r_mat.end());
    size_t sz = static_cast<size_t>(std::sqrt(mat.size()));
    floyd_warshall_impl(mat.data(), sz, sz);
    rust::vec<double> r_v;
    std::copy(mat.begin(), mat.end(), std::back_inserter(r_v));
    return r_v;
}
