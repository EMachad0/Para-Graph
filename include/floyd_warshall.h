#pragma once
#include "framework-rs/src/bridge.rs.h"
#include "rust/cxx.h"

rust::vec<double> floyd_warshall(rust::vec<double> mat);
