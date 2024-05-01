#include "para-graph/include/prefix_sum.h"

void prefix_sum(const size_t n, rust::slice<double> arr) {
    double res[n];
    for (size_t i = 0; i < n; ++i) res[i] = arr[i];

    size_t e = ceil(log2(n));
    size_t pot = 1;
    for (size_t i = 0; i < e; i++) {
        #pragma acc parallel loop
        for (size_t j = n-1; j >= pot; j--) {
            res[j] += res[j - pot];
        }
        pot <<= 1;
    }

    for (size_t i = 0; i < n; ++i) arr[i] = res[i];
}

