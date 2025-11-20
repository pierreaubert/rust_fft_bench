# FFT Benchmark Comparison

This project benchmarks various FFT implementations in Rust, focusing on:

- **rustfft**: Popular pure-Rust FFT library
- **scirs2**: Scientific computing library (SciPy-like) with FFT support

## Benchmark Parameters

- **Sizes**: 1024, 2048, 4096, 8192
- **Data types**: Complex64 (f32) and Complex128 (f64)

## Running Benchmarks

### Quick Start

```bash
# Run the benchmark script (recommended)
./run_benchmarks.sh
```

### Manual Execution

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark group
cargo bench rustfft_complex32
cargo bench scirs_complex32

# Run comparison benchmark (4096 size)
cargo bench fft_comparison_4096

# Generate detailed report with verbose output
cargo bench -- --verbose
```

## Running Tests

```bash
# Run all tests
cargo test

# Run tests in release mode
cargo test --release
```

## Code Quality

```bash
# Run clippy linter
cargo clippy --all-targets -- -D warnings

# Format code
cargo fmt

# Check formatting without modifying
cargo fmt -- --check
```

## Benchmark Details

### What's Being Measured

- **rustfft_complex32**: rustfft with Complex32 (f32 precision)
- **rustfft_complex64**: rustfft with Complex64 (f64 precision)
- **scirs_complex32**: scirs2 with f32 input (outputs Complex64)
- **scirs_complex64**: scirs2 with f64 input (outputs Complex64)

### Test Sizes

Each implementation is tested with FFT sizes: 1024, 2048, 4096, 8192

### Comparison Group

The `fft_comparison_4096` benchmark runs all implementations at size 4096 for direct comparison.

## Results

Benchmark results will be saved in `target/criterion/` with HTML reports.

To view results:

```bash
open target/criterion/report/index.html
```

## Performance Notes

- **rustfft**: Pure Rust implementation, optimized for various sizes
- **scirs2**: SciPy-compatible library with SIMD optimizations, always uses f64 precision internally
- Both libraries support power-of-2 sizes efficiently
- scirs2 includes plan caching and can leverage GPU acceleration (if configured)

## Build Requirements

- Rust 1.70 or later
- Cargo

## Dependencies

- rustfft: 6.2
- scirs2: 0.1.0-rc.2
- num-complex: 0.4
- criterion: 0.5 (benchmarking framework)
