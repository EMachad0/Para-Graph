#pragma once
#include "para-graph/src/bridge.rs.h"
#include "rust/cxx.h"
#include <vector>
#include <stdexcept>

std::unique_ptr<std::vector<double>> gaussian_elimination(const size_t n, rust::slice<const rust::Vec<double>> mat);
