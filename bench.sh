#!/bin/bash

RUNS=10
WARMUP=3
PARAMETERS="num_threads 1 16"

# Prepare execution
ID=$(date +%y%m%d%H%M%S)

OUT=out/$ID
LOG=log/$ID

mkdir -p "$OUT"
mkdir -p "$LOG"

echo "ID" "$ID"

# Sequential
RUST_SETUP="cargo build --release"
RUST="RAYON_NUM_THREADS={num_threads} ./target/release/framework-rs > $OUT/bench.out"

# Benchmarks
hyperfine -s "$RUST_SETUP" -r $RUNS -w $WARMUP -P $PARAMETERS "$RUST" --export-markdown $LOG/log_rust.md --export-csv $LOG/log_rust.csv --export-json $LOG/log_rust.json --show-output

# Generate plots
./plot.sh "$ID"