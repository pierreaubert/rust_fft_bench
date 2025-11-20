# FFT Benchmark Implementation Details

## Overview

This benchmark suite compares two FFT implementations in Rust:

1. **rustfft** - A pure Rust FFT library
2. **scirs2** - A comprehensive scientific computing library (SciPy-like for Rust)

## Implementation Details

### rustfft

- Uses `FftPlanner` to create optimized FFT plans
- Supports both `Complex32` (f32) and `Complex64` (f64)
- In-place processing with `process()` method
- Highly optimized for power-of-2 sizes

### scirs2

- Part of the SciRS2 scientific computing ecosystem
- Always returns `Complex64` (f64) regardless of input type
- Supports plan caching for repeated transforms
- Includes SIMD optimizations
- Can leverage GPU acceleration (when configured)

## Benchmark Methodology

### Input Data

All benchmarks use sine wave input:
```rust
let input: Vec<T> = (0..size)
    .map(|i| (i as T).sin())
    .collect();
```

### Sizes Tested

- 1024 points
- 2048 points
- 4096 points
- 8192 points

All sizes are powers of 2, which are optimal for FFT algorithms.

### Metrics

Criterion.rs provides:
- **Mean execution time**: Average time per iteration
- **Standard deviation**: Variability in measurements
- **Throughput**: Operations per second
- **Comparison**: Performance relative to previous runs

## Test Coverage

### Correctness Tests

1. **Size verification**: Ensures output length matches input
2. **Parseval's theorem**: Verifies energy conservation
   - Sum of squares in time domain = sum of squares in frequency domain / N

### Example Test

```rust
#[test]
fn test_fft_parseval_theorem_rustfft() {
    let size = 128;
    let input: Vec<Complex32> = (0..size)
        .map(|i| Complex32::new((i as f32).sin(), 0.0))
        .collect();
    
    let time_energy: f32 = input.iter()
        .map(|c| c.norm_sqr())
        .sum();
    
    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(size);
    let mut buffer = input.clone();
    fft.process(&mut buffer);
    
    let freq_energy: f32 = buffer.iter()
        .map(|c| c.norm_sqr())
        .sum::<f32>() / (size as f32);
    
    assert!((time_energy - freq_energy).abs() / time_energy < 0.01);
}
```

## Expected Performance Characteristics

### rustfft

- **Strengths**: 
  - Pure Rust, no external dependencies
  - Excellent performance for power-of-2 sizes
  - Low memory overhead
  - Both f32 and f64 support

- **Considerations**:
  - CPU-only implementation
  - Manual plan management

### scirs2

- **Strengths**:
  - Part of comprehensive scientific computing ecosystem
  - Automatic plan caching
  - SIMD optimizations
  - GPU acceleration support
  - SciPy-compatible API

- **Considerations**:
  - Always uses f64 internally
  - Larger dependency footprint
  - More complex setup for GPU features

## Running the Benchmarks

### Quick Run

```bash
./run_benchmarks.sh
```

### Detailed Analysis

```bash
# Run with verbose output
cargo bench -- --verbose

# Run specific size
cargo bench 4096

# Save baseline for comparison
cargo bench -- --save-baseline my_baseline

# Compare against baseline
cargo bench -- --baseline my_baseline
```

## Interpreting Results

Criterion generates HTML reports in `target/criterion/report/index.html`.

Key metrics to compare:
1. **Mean time**: Lower is better
2. **Throughput**: Higher is better
3. **Consistency**: Lower standard deviation is better

## Notes

- All benchmarks run in release mode with optimizations
- Results may vary based on CPU architecture and system load
- For fair comparison, close other applications during benchmarking
- Run multiple times to ensure consistent results
