#include "para-graph/include/prefix_sum.h"

void prefix_sum(const size_t n, rust::slice<double> mat) {
    size_t e = floor(log2(n));
    size_t pot = 1;
    for (size_t i = 0; i <= e; i++) {
        #pragma acc loop
        for (size_t j = n-1; j >= pot; j--) {
            mat[j] += mat[j - pot];
        }
        pot <<= 1;
    }
}

