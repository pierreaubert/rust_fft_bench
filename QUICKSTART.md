# Quick Start Guide

## Installation

This project requires Rust 1.70 or later. If you don't have Rust installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Running Benchmarks

### Option 1: Use the provided script (Recommended)

```bash
./run_benchmarks.sh
```

This will:
1. Build the project in release mode
2. Run all tests to verify correctness
3. Execute all benchmarks
4. Generate HTML reports

### Option 2: Manual execution

```bash
# Build and test
cargo build --release
cargo test --release

# Run benchmarks
cargo bench
```

## Viewing Results

After running benchmarks, open the HTML report:

```bash
open target/criterion/report/index.html
```

## Project Structure

```
fft_bench/
├── src/
│   ├── lib.rs              # Library root
│   └── benchmarks.rs       # FFT implementations and tests
├── benches/
│   └── fft_comparison.rs   # Criterion benchmark definitions
├── Cargo.toml              # Dependencies and configuration
├── README.md               # Main documentation
├── BENCHMARKS.md           # Detailed benchmark information
└── run_benchmarks.sh       # Automated benchmark runner
```

## What Gets Benchmarked

### Libraries
- **rustfft 6.2**: Pure Rust FFT implementation
- **scirs2 0.1.0-rc.2**: SciPy-like scientific computing library

### Test Configurations
- FFT sizes: 1024, 2048, 4096, 8192
- Data types: f32 and f64 inputs
- Both forward FFT operations

### Benchmark Groups
1. `rustfft_complex32` - rustfft with f32 precision
2. `rustfft_complex64` - rustfft with f64 precision
3. `scirs_complex32` - scirs2 with f32 input
4. `scirs_complex64` - scirs2 with f64 input
5. `fft_comparison_4096` - Direct comparison at size 4096

## Understanding Results

Criterion provides several metrics:

- **Time**: Mean execution time (lower is better)
- **Throughput**: Operations per second (higher is better)
- **Change**: Performance change from previous run
- **R²**: Goodness of fit (closer to 1.0 is better)

## Next Steps

1. Run the benchmarks on your system
2. Compare the results between rustfft and scirs2
3. Try different FFT sizes by modifying `benches/fft_comparison.rs`
4. Experiment with different input patterns in `src/benchmarks.rs`

## Troubleshooting

### Build Issues

If you encounter build errors:

```bash
# Update Rust toolchain
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

### Performance Issues

For accurate benchmarks:
- Close other applications
- Disable CPU frequency scaling if possible
- Run multiple times to ensure consistency
- Use `--save-baseline` to track changes over time

## Code Quality

The project includes linting and formatting tools:

```bash
# Check code style
cargo clippy --all-targets -- -D warnings

# Format code
cargo fmt

# Run all quality checks
cargo fmt -- --check && cargo clippy --all-targets -- -D warnings && cargo test
```

## Contributing

To add new FFT libraries:

1. Add dependency to `Cargo.toml`
2. Implement wrapper function in `src/benchmarks.rs`
3. Add test in the `tests` module
4. Add benchmark in `benches/fft_comparison.rs`
5. Update documentation

## Support

For issues or questions:
- Check `README.md` for detailed documentation
- Review `BENCHMARKS.md` for implementation details
- Examine the source code in `src/benchmarks.rs`
