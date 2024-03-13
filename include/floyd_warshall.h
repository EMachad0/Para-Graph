#pragma once
#include "framework-rs/src/bridge.rs.h"
#include "rust/cxx.h"

void floyd_warshall(const size_t n, rust::slice<double> mat);
//void floyd_warshall(const size_t n);
