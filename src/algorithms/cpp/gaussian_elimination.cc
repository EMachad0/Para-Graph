#include "para-graph/include/gaussian_elimination.h"

const double EPS = 1e-10;

std::unique_ptr<std::vector<double>> gaussian_elimination(const size_t n, rust::Slice<const rust::Vec<double>> rust_mat) {
    double mat[n][n+1];
    for (size_t i = 0; i < n; i++) {
        for (size_t j = 0; j < n+1; j++) {
            mat[i][j] = rust_mat[i][j];
        }
    }
    
    for (size_t col = 0; col < n; col++) {
        size_t max_row = col;
        for (size_t i = col + 1; i < n; i++) {
            if (std::abs(mat[i][col]) > std::abs(mat[max_row][col])) {
                max_row = i;
            }
        }
        if (max_row != col) {
            #pragma acc parallel loop
            for (size_t i = col; i <= n; i++) {
                double temp = mat[col][i];
                mat[col][i] = mat[max_row][i];
                mat[max_row][i] = temp;
            }
        }
        if (std::abs(mat[col][col]) < EPS) {
            throw std::runtime_error("Matrix is singular");
        }
        
        #pragma acc parallel loop
        for (size_t i = 0; i < n; i++) {
            if (i == col) {
                continue;
            }
            double ratio = mat[i][col] / mat[col][col];
            #pragma acc loop
            for (size_t j = col; j <= n; j++) {
                mat[i][j] -= ratio * mat[col][j];
            }
        }
    }
    
    std::vector<double> result(n);
    for (size_t i = 0; i < n; i++) {
        result[i] = mat[i][n] / mat[i][i];
    }
    return std::make_unique<std::vector<double>>(result);
}
