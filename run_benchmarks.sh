#!/bin/bash

# FFT Benchmark Runner Script
# This script runs all benchmarks and generates reports

set -e

echo "==================================="
echo "FFT Performance Benchmark Suite"
echo "==================================="
echo ""

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    echo "Error: cargo not found. Please install Rust."
    exit 1
fi

# Build in release mode
echo "Building project in release mode..."
cargo build --release
echo ""

# Run tests first
echo "Running tests to verify correctness..."
cargo test --release
echo ""

# Run benchmarks
echo "Running benchmarks..."
echo "This may take several minutes..."
echo ""

cargo bench

echo ""
echo "==================================="
echo "Benchmark Complete!"
echo "==================================="
echo ""
echo "Results saved to: target/criterion/"
echo "Open target/criterion/report/index.html to view detailed results"
echo ""
