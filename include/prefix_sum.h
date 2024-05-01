#pragma once
#include "para-graph/src/bridge.rs.h"
#include "rust/cxx.h"
#include <cmath>

void prefix_sum(const size_t n, rust::slice<double> arr);
